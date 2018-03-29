//! Types for the `SNS` service.

/// The [`AWS::SNS::Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-subscription.html) resource type.
#[derive(Debug)]
pub struct Subscription {
    properties: SubscriptionProperties
}

/// Properties for the `Subscription` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionProperties {
    /// Property `Endpoint`.
    #[serde(rename="Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Property `Protocol`.
    #[serde(rename="Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Property `TopicArn`.
    #[serde(rename="TopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

impl<'a> ::Resource<'a> for Subscription {
    type Properties = SubscriptionProperties;
    const TYPE: &'static str = "AWS::SNS::Subscription";
    fn properties(&self) -> &SubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Subscription {}

impl From<SubscriptionProperties> for Subscription {
    fn from(properties: SubscriptionProperties) -> Subscription {
        Subscription { properties }
    }
}

/// The [`AWS::SNS::Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic.html) resource type.
#[derive(Debug)]
pub struct Topic {
    properties: TopicProperties
}

/// Properties for the `Topic` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TopicProperties {
    /// Property `DisplayName`.
    #[serde(rename="DisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Property `Subscription`.
    #[serde(rename="Subscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Vec<self::topic::Subscription>>,
    /// Property `TopicName`.
    #[serde(rename="TopicName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

impl<'a> ::Resource<'a> for Topic {
    type Properties = TopicProperties;
    const TYPE: &'static str = "AWS::SNS::Topic";
    fn properties(&self) -> &TopicProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Topic {}

impl From<TopicProperties> for Topic {
    fn from(properties: TopicProperties) -> Topic {
        Topic { properties }
    }
}

/// The [`AWS::SNS::TopicPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html) resource type.
#[derive(Debug)]
pub struct TopicPolicy {
    properties: TopicPolicyProperties
}

/// Properties for the `TopicPolicy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TopicPolicyProperties {
    /// Property `PolicyDocument`.
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
    /// Property `Topics`.
    #[serde(rename="Topics")]
    pub topics: Vec<String>,
}

impl<'a> ::Resource<'a> for TopicPolicy {
    type Properties = TopicPolicyProperties;
    const TYPE: &'static str = "AWS::SNS::TopicPolicy";
    fn properties(&self) -> &TopicPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TopicPolicy {}

impl From<TopicPolicyProperties> for TopicPolicy {
    fn from(properties: TopicPolicyProperties) -> TopicPolicy {
        TopicPolicy { properties }
    }
}

pub mod topic {
    //! Property types for the `Topic` resource.

    /// The [`AWS::SNS::Topic.Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-subscription.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Subscription {
        /// Property `Endpoint`.
        #[serde(rename="Endpoint")]
        pub endpoint: String,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        pub protocol: String,
    }
}
