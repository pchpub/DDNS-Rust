use std::fmt::Display;

#[derive(Debug)]
pub enum ProvidersErrorType {
    QueryDomainRecordsError,
    DeleteSubdomainRecordsError,
    NoRecordFound,
    TooManyRecords,
    DeleteDomainRecordsError,
    NotInitialized,
    KeyError,
    NetworkError,
    OtherError,
}

impl Display for ProvidersErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvidersErrorType::QueryDomainRecordsError => {
                write!(f, "QueryDomainRecordsError")
            }
            ProvidersErrorType::NoRecordFound => write!(f, "NoRecordFound"),
            ProvidersErrorType::TooManyRecords => write!(f, "TooManyRecords"),
            ProvidersErrorType::NotInitialized => write!(f, "NotInitialized"),
            ProvidersErrorType::KeyError => write!(f, "KeyError"),
            ProvidersErrorType::NetworkError => write!(f, "NetworkError"),
            ProvidersErrorType::OtherError => write!(f, "OtherError"),
            ProvidersErrorType::DeleteSubdomainRecordsError => {
                write!(f, "DeleteSubdomainRecordsError")
            }
            ProvidersErrorType::DeleteDomainRecordsError => write!(f, "DeleteDomainRecordsError"),
        }
    }
}
