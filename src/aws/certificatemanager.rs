//! Types for the `CertificateManager` service.

/// The [`AWS::CertificateManager::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-certificatemanager-certificate.html) resource type.
#[derive(Debug)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateProperties {
    /// Property `DomainName`.
    #[serde(rename="DomainName")]
    pub domain_name: String,
    /// Property `DomainValidationOptions`.
    #[serde(rename="DomainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<self::certificate::DomainValidationOption>>,
    /// Property `SubjectAlternativeNames`.
    #[serde(rename="SubjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
}

impl<'a> ::Resource<'a> for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::CertificateManager::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Certificate {}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

pub mod certificate {
    //! Property types for the `Certificate` resource.

    /// The [`AWS::CertificateManager::Certificate.DomainValidationOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-certificatemanager-certificate-domainvalidationoption.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DomainValidationOption {
        /// Property `DomainName`.
        #[serde(rename="DomainName")]
        pub domain_name: String,
        /// Property `ValidationDomain`.
        #[serde(rename="ValidationDomain")]
        pub validation_domain: String,
    }
}
