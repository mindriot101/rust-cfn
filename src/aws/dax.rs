/// The [`AWS::DAX::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-cluster.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename="AvailabilityZones")]
    pub availability_zones: (),
    #[serde(rename="ClusterName")]
    pub cluster_name: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="IAMRoleARN")]
    pub iam_role_arn: (),
    #[serde(rename="NodeType")]
    pub node_type: (),
    #[serde(rename="NotificationTopicARN")]
    pub notification_topic_arn: (),
    #[serde(rename="ParameterGroupName")]
    pub parameter_group_name: (),
    #[serde(rename="PreferredMaintenanceWindow")]
    pub preferred_maintenance_window: (),
    #[serde(rename="ReplicationFactor")]
    pub replication_factor: (),
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: (),
    #[serde(rename="SubnetGroupName")]
    pub subnet_group_name: (),
    #[serde(rename="Tags")]
    pub tags: (),
}

impl<'a> ::Resource<'a> for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::DAX::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::DAX::ParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-parametergroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ParameterGroup {
    properties: ParameterGroupProperties
}

/// Properties for the `ParameterGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct ParameterGroupProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="ParameterGroupName")]
    pub parameter_group_name: (),
    #[serde(rename="ParameterNameValues")]
    pub parameter_name_values: (),
}

impl<'a> ::Resource<'a> for ParameterGroup {
    type Properties = ParameterGroupProperties;
    const TYPE: &'static str = "AWS::DAX::ParameterGroup";
    fn properties(&self) -> &ParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ParameterGroupProperties {
        &mut self.properties
    }
}

impl From<ParameterGroupProperties> for ParameterGroup {
    fn from(properties: ParameterGroupProperties) -> ParameterGroup {
        ParameterGroup { properties }
    }
}

/// The [`AWS::DAX::SubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dax-subnetgroup.html) resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetGroup {
    properties: SubnetGroupProperties
}

/// Properties for the `SubnetGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct SubnetGroupProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="SubnetGroupName")]
    pub subnet_group_name: (),
    #[serde(rename="SubnetIds")]
    pub subnet_ids: (),
}

impl<'a> ::Resource<'a> for SubnetGroup {
    type Properties = SubnetGroupProperties;
    const TYPE: &'static str = "AWS::DAX::SubnetGroup";
    fn properties(&self) -> &SubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SubnetGroupProperties {
        &mut self.properties
    }
}

impl From<SubnetGroupProperties> for SubnetGroup {
    fn from(properties: SubnetGroupProperties) -> SubnetGroup {
        SubnetGroup { properties }
    }
}

