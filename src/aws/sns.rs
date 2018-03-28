/// The [`AWS::SNS::Subscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-subscription.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Subscription {
    properties: SubscriptionProperties
}

/// Properties for the `Subscription` resource.
#[derive(Serialize, Deserialize)]
pub struct SubscriptionProperties {
    #[serde(rename="Endpoint")]
    pub endpoint: (),
    #[serde(rename="Protocol")]
    pub protocol: (),
    #[serde(rename="TopicArn")]
    pub topic_arn: (),
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

impl From<SubscriptionProperties> for Subscription {
    fn from(properties: SubscriptionProperties) -> Subscription {
        Subscription { properties }
    }
}

/// The [`AWS::SNS::Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Topic {
    properties: TopicProperties
}

/// Properties for the `Topic` resource.
#[derive(Serialize, Deserialize)]
pub struct TopicProperties {
    #[serde(rename="DisplayName")]
    pub display_name: (),
    #[serde(rename="Subscription")]
    pub subscription: (),
    #[serde(rename="TopicName")]
    pub topic_name: (),
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

impl From<TopicProperties> for Topic {
    fn from(properties: TopicProperties) -> Topic {
        Topic { properties }
    }
}

/// The [`AWS::SNS::TopicPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-policy.html) resource.
#[derive(Serialize, Deserialize)]
pub struct TopicPolicy {
    properties: TopicPolicyProperties
}

/// Properties for the `TopicPolicy` resource.
#[derive(Serialize, Deserialize)]
pub struct TopicPolicyProperties {
    #[serde(rename="PolicyDocument")]
    pub policy_document: (),
    #[serde(rename="Topics")]
    pub topics: (),
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

impl From<TopicPolicyProperties> for TopicPolicy {
    fn from(properties: TopicPolicyProperties) -> TopicPolicy {
        TopicPolicy { properties }
    }
}

