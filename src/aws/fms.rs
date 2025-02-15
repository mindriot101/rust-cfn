//! Types for the `FMS` service.

/// The [`AWS::FMS::NotificationChannel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-notificationchannel.html) resource type.
#[derive(Debug, Default)]
pub struct NotificationChannel {
    properties: NotificationChannelProperties,
}

/// Properties for the `NotificationChannel` resource.
#[derive(Debug, Default)]
pub struct NotificationChannelProperties {
    /// Property [`SnsRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-notificationchannel.html#cfn-fms-notificationchannel-snsrolename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_role_name: ::Value<String>,
    /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-notificationchannel.html#cfn-fms-notificationchannel-snstopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_arn: ::Value<String>,
}

impl ::serde::Serialize for NotificationChannelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsRoleName", &self.sns_role_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", &self.sns_topic_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NotificationChannelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(
        d: D,
    ) -> Result<NotificationChannelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NotificationChannelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NotificationChannelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut sns_role_name: Option<::Value<String>> = None;
                let mut sns_topic_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "SnsRoleName" => {
                            sns_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicArn" => {
                            sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NotificationChannelProperties {
                    sns_role_name: sns_role_name
                        .ok_or(::serde::de::Error::missing_field("SnsRoleName"))?,
                    sns_topic_arn: sns_topic_arn
                        .ok_or(::serde::de::Error::missing_field("SnsTopicArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for NotificationChannel {
    type Properties = NotificationChannelProperties;
    const TYPE: &'static str = "AWS::FMS::NotificationChannel";
    fn properties(&self) -> &NotificationChannelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NotificationChannelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NotificationChannel {}

impl From<NotificationChannelProperties> for NotificationChannel {
    fn from(properties: NotificationChannelProperties) -> NotificationChannel {
        NotificationChannel { properties }
    }
}

/// The [`AWS::FMS::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html) resource type.
#[derive(Debug, Default)]
pub struct Policy {
    properties: PolicyProperties,
}

/// Properties for the `Policy` resource.
#[derive(Debug, Default)]
pub struct PolicyProperties {
    /// Property [`DeleteAllPolicyResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-deleteallpolicyresources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delete_all_policy_resources: Option<::Value<bool>>,
    /// Property [`ExcludeMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-excludemap).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub exclude_map: Option<::Value<self::policy::IEMap>>,
    /// Property [`ExcludeResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-excluderesourcetags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub exclude_resource_tags: ::Value<bool>,
    /// Property [`IncludeMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-includemap).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub include_map: Option<::Value<self::policy::IEMap>>,
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-policyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_name: ::Value<String>,
    /// Property [`RemediationEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-remediationenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub remediation_enabled: ::Value<bool>,
    /// Property [`ResourceTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-resourcetags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_tags: Option<::ValueList<self::policy::ResourceTag>>,
    /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-resourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_type: ::Value<String>,
    /// Property [`ResourceTypeList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-resourcetypelist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_type_list: Option<::ValueList<String>>,
    /// Property [`ResourcesCleanUp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-resourcescleanup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resources_clean_up: Option<::Value<bool>>,
    /// Property [`SecurityServicePolicyData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-securityservicepolicydata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_service_policy_data: ::Value<::json::Value>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html#cfn-fms-policy-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::policy::PolicyTag>>,
}

impl ::serde::Serialize for PolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref delete_all_policy_resources) = self.delete_all_policy_resources {
            ::serde::ser::SerializeMap::serialize_entry(
                &mut map,
                "DeleteAllPolicyResources",
                delete_all_policy_resources,
            )?;
        }
        if let Some(ref exclude_map) = self.exclude_map {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeMap", exclude_map)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(
            &mut map,
            "ExcludeResourceTags",
            &self.exclude_resource_tags,
        )?;
        if let Some(ref include_map) = self.include_map {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeMap", include_map)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(
            &mut map,
            "RemediationEnabled",
            &self.remediation_enabled,
        )?;
        if let Some(ref resource_tags) = self.resource_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceTags", resource_tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
        if let Some(ref resource_type_list) = self.resource_type_list {
            ::serde::ser::SerializeMap::serialize_entry(
                &mut map,
                "ResourceTypeList",
                resource_type_list,
            )?;
        }
        if let Some(ref resources_clean_up) = self.resources_clean_up {
            ::serde::ser::SerializeMap::serialize_entry(
                &mut map,
                "ResourcesCleanUp",
                resources_clean_up,
            )?;
        }
        ::serde::ser::SerializeMap::serialize_entry(
            &mut map,
            "SecurityServicePolicyData",
            &self.security_service_policy_data,
        )?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut delete_all_policy_resources: Option<::Value<bool>> = None;
                let mut exclude_map: Option<::Value<self::policy::IEMap>> = None;
                let mut exclude_resource_tags: Option<::Value<bool>> = None;
                let mut include_map: Option<::Value<self::policy::IEMap>> = None;
                let mut policy_name: Option<::Value<String>> = None;
                let mut remediation_enabled: Option<::Value<bool>> = None;
                let mut resource_tags: Option<::ValueList<self::policy::ResourceTag>> = None;
                let mut resource_type: Option<::Value<String>> = None;
                let mut resource_type_list: Option<::ValueList<String>> = None;
                let mut resources_clean_up: Option<::Value<bool>> = None;
                let mut security_service_policy_data: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<self::policy::PolicyTag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeleteAllPolicyResources" => {
                            delete_all_policy_resources =
                                ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExcludeMap" => {
                            exclude_map = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExcludeResourceTags" => {
                            exclude_resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IncludeMap" => {
                            include_map = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RemediationEnabled" => {
                            remediation_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceTags" => {
                            resource_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceType" => {
                            resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceTypeList" => {
                            resource_type_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourcesCleanUp" => {
                            resources_clean_up = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityServicePolicyData" => {
                            security_service_policy_data =
                                ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyProperties {
                    delete_all_policy_resources: delete_all_policy_resources,
                    exclude_map: exclude_map,
                    exclude_resource_tags: exclude_resource_tags
                        .ok_or(::serde::de::Error::missing_field("ExcludeResourceTags"))?,
                    include_map: include_map,
                    policy_name: policy_name
                        .ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    remediation_enabled: remediation_enabled
                        .ok_or(::serde::de::Error::missing_field("RemediationEnabled"))?,
                    resource_tags: resource_tags,
                    resource_type: resource_type
                        .ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                    resource_type_list: resource_type_list,
                    resources_clean_up: resources_clean_up,
                    security_service_policy_data: security_service_policy_data.ok_or(
                        ::serde::de::Error::missing_field("SecurityServicePolicyData"),
                    )?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Policy {
    type Properties = PolicyProperties;
    const TYPE: &'static str = "AWS::FMS::Policy";
    fn properties(&self) -> &PolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Policy {}

impl From<PolicyProperties> for Policy {
    fn from(properties: PolicyProperties) -> Policy {
        Policy { properties }
    }
}

pub mod policy {
    //! Property types for the `Policy` resource.

    /// The [`AWS::FMS::Policy.IEMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-iemap.html) property type.
    #[derive(Debug, Default)]
    pub struct IEMap {
        /// Property [`ACCOUNT`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-iemap.html#cfn-fms-policy-iemap-account).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account: Option<::ValueList<String>>,
        /// Property [`ORGUNIT`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-iemap.html#cfn-fms-policy-iemap-orgunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub orgunit: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for IEMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref account) = self.account {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ACCOUNT", account)?;
            }
            if let Some(ref orgunit) = self.orgunit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ORGUNIT", orgunit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IEMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IEMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IEMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IEMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(
                    self,
                    mut map: A,
                ) -> Result<Self::Value, A::Error> {
                    let mut account: Option<::ValueList<String>> = None;
                    let mut orgunit: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) =
                        ::serde::de::MapAccess::next_key::<String>(&mut map)?
                    {
                        match __cfn_key.as_ref() {
                            "ACCOUNT" => {
                                account = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ORGUNIT" => {
                                orgunit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IEMap {
                        account: account,
                        orgunit: orgunit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FMS::Policy.PolicyTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-policytag.html) property type.
    #[derive(Debug, Default)]
    pub struct PolicyTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-policytag.html#cfn-fms-policy-policytag-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-policytag.html#cfn-fms-policy-policytag-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for PolicyTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PolicyTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PolicyTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PolicyTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(
                    self,
                    mut map: A,
                ) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) =
                        ::serde::de::MapAccess::next_key::<String>(&mut map)?
                    {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PolicyTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FMS::Policy.ResourceTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-resourcetag.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-resourcetag.html#cfn-fms-policy-resourcetag-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-resourcetag.html#cfn-fms-policy-resourcetag-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ResourceTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(
                    self,
                    mut map: A,
                ) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) =
                        ::serde::de::MapAccess::next_key::<String>(&mut map)?
                    {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
