use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat, Utc};
use reqwest::Client;
use thiserror::Error;

const URL: &str = "https://api.opentransportdata.swiss/ojp20";

pub enum RequestType {
    LocationInformation,
    Trip,
    StopEvent,
    Unknown,
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("Missing authetification token")]
    MissingAuthToken,
    #[error("Missing location name")]
    MissingLocationName,
    #[error("Missing from and to ids")]
    MissingFromAndToId,
    #[error("Missing from id")]
    MissingFromId,
    #[error("Missing to id")]
    MissingToId,
    #[error("Unknown request type: must be LocationInformation, Trip, or StopEvent")]
    UnknownRequestType,
    #[error("Events type is not implemented")]
    EventsRequestTypeNotImplemented,
    #[error("Invalid number of results, got {0}, should be > 0.")]
    InvalidNumberResults(u32),
    #[error("Http request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

impl TryFrom<RequestType> for String {
    type Error = RequestError;
    fn try_from(value: RequestType) -> Result<Self, Self::Error> {
        match value {
            RequestType::LocationInformation => Ok("OJPLocationInformationRequest".to_string()),
            RequestType::Trip => Ok("OJPTripRequest".to_string()),
            RequestType::StopEvent => Ok("OJPStopEventRequest".to_string()),
            RequestType::Unknown => Err(RequestError::UnknownRequestType),
        }
    }
}

pub struct RequestBuilder {
    token: Option<String>,
    date_time: DateTime<Utc>,
    request_type: RequestType,
    number_results: u32,
    from: Option<i32>,
    to: Option<i32>,
    name: Option<String>,
    requestor_ref: String,
}

impl RequestBuilder {
    pub fn new(date_time: NaiveDateTime) -> Self {
        // We convert NaiveDateTime to Utc through Local (for the offset)
        // First we get the "now" local time (used for the offset)
        // and add it to the NaiveDateTime
        let date_time = date_time
            .and_local_timezone(*Local::now().offset())
            .unwrap();

        let date_time = date_time.to_utc();
        RequestBuilder {
            date_time,
            token: None,
            request_type: RequestType::Unknown,
            number_results: 0,
            from: None,
            to: None,
            name: None,
            requestor_ref: String::new(),
        }
    }

    pub fn set_from(mut self, from: i32) -> Self {
        self.from = Some(from);
        self
    }

    pub fn set_to(mut self, to: i32) -> Self {
        self.to = Some(to);
        self
    }

    pub fn set_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn set_request_type(mut self, request_type: RequestType) -> Self {
        self.request_type = request_type;
        self
    }

    pub fn set_number_results(mut self, number_results: u32) -> Self {
        self.number_results = number_results;
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn set_requestor_ref(mut self, requestor_ref: &str) -> Self {
        self.requestor_ref = requestor_ref.to_string();
        self
    }

    pub fn try_request_body(&self) -> Result<String, RequestError> {
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let date_time = self.date_time.to_rfc3339_opts(SecondsFormat::Millis, true);

        let number_results = self.number_results;
        match self.request_type {
            RequestType::Unknown => Err(RequestError::UnknownRequestType),
            RequestType::LocationInformation => {
                if number_results == 0 {
                    return Err(RequestError::InvalidNumberResults(number_results));
                }
                if self.name.is_none() {
                    return Err(RequestError::MissingLocationName);
                }
                let req = format!(
"<?xml version=\"1.0\" encoding=\"UTF-8\"?>
                            <OJP xmlns=\"http://www.vdv.de/ojp\" xmlns:siri=\"http://www.siri.org.uk/siri\" version=\"2.0\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.vdv.de/ojp ../../../../Downloads/OJP-changes_for_v1.1%20(1)/OJP-changes_for_v1.1/OJP.xsd\">
                             	<OJPRequest>
                                    <siri:ServiceRequest>
                                        <siri:RequestTimestamp>{now}</siri:RequestTimestamp>
                                        <siri:RequestorRef>{}</siri:RequestorRef>
                                        <OJPLocationInformationRequest>
                                        <siri:RequestTimestamp>{now}</siri:RequestTimestamp>
                                        <siri:MessageIdentifier>LIR-1a</siri:MessageIdentifier>
                                        <InitialInput>
                                            <Name>{}</Name>
                                        </InitialInput>
                                        <Restrictions>
                                            <Type>stop</Type>
                                            <NumberOfResults>{number_results}</NumberOfResults>
                                        </Restrictions>
                                    </OJPLocationInformationRequest>
                                    </siri:ServiceRequest>
                                </OJPRequest>
                            </OJP>", self.requestor_ref, self.name.as_ref().unwrap());
                Ok(req)
            }
            RequestType::StopEvent => Err(RequestError::EventsRequestTypeNotImplemented),
            RequestType::Trip => {
                if number_results == 0 {
                    return Err(RequestError::InvalidNumberResults(number_results));
                }
                let (from, to) = match (self.from, self.to) {
                    (Some(from), Some(to)) => (from, to),
                    (None, None) => return Err(RequestError::MissingFromAndToId),
                    (Some(_), None) => return Err(RequestError::MissingToId),
                    (None, Some(_)) => return Err(RequestError::MissingFromId),
                };
                let req = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
                            <OJP xmlns=\"http://www.vdv.de/ojp\" xmlns:siri=\"http://www.siri.org.uk/siri\" version=\"2.0\">
                             	<OJPRequest>
                                    <siri:ServiceRequest>
                                        <siri:RequestTimestamp>{now}</siri:RequestTimestamp>
                                        <siri:RequestorRef>{}</siri:RequestorRef>
                                        <OJPTripRequest>
                                            <siri:RequestTimestamp>{now}</siri:RequestTimestamp>
                                            <siri:MessageIdentifier>TR-1r1</siri:MessageIdentifier>
                                            <Origin>
                                                <PlaceRef>
                                                    <siri:StopPointRef>{from}</siri:StopPointRef>
                                                </PlaceRef>
                                                <DepArrTime>{date_time}</DepArrTime>
                                            </Origin>
                                            <Destination>
                                                <PlaceRef>
                                                    <siri:StopPointRef>{to}</siri:StopPointRef>
                                                </PlaceRef>
                                            </Destination>
                                            <Params>
                                                <NumberOfResults>{number_results}</NumberOfResults>
                                            </Params>
                                        </OJPTripRequest>
                                    </siri:ServiceRequest>
                                </OJPRequest>
                            </OJP>", self.requestor_ref);
                Ok(req)
            }
        }
    }

    pub fn build_request(self) -> Result<reqwest::RequestBuilder, RequestError> {
        let id_request = self.try_request_body()?;

        if self.token.is_none() {
            return Err(RequestError::MissingAuthToken);
        }

        let req = Client::new()
            .post(URL)
            .header("Content-Type", "application/xml")
            .header("accept", "*/*")
            .bearer_auth(self.token.as_ref().unwrap())
            .body(id_request);
        Ok(req)
    }

    pub async fn send_request(self) -> Result<String, RequestError> {
        let respone = self.build_request()?.send().await?.text().await?;
        Ok(respone)
    }
}
