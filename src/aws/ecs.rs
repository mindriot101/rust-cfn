/// The [`AWS::ECS::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename="ClusterName")]
    pub cluster_name: (),
}

impl<'a> ::Resource<'a> for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::ECS::Cluster";
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

/// The [`AWS::ECS::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Serialize, Deserialize)]
pub struct ServiceProperties {
    #[serde(rename="Cluster")]
    pub cluster: (),
    #[serde(rename="DeploymentConfiguration")]
    pub deployment_configuration: (),
    #[serde(rename="DesiredCount")]
    pub desired_count: (),
    #[serde(rename="HealthCheckGracePeriodSeconds")]
    pub health_check_grace_period_seconds: (),
    #[serde(rename="LaunchType")]
    pub launch_type: (),
    #[serde(rename="LoadBalancers")]
    pub load_balancers: (),
    #[serde(rename="NetworkConfiguration")]
    pub network_configuration: (),
    #[serde(rename="PlacementConstraints")]
    pub placement_constraints: (),
    #[serde(rename="PlacementStrategies")]
    pub placement_strategies: (),
    #[serde(rename="PlatformVersion")]
    pub platform_version: (),
    #[serde(rename="Role")]
    pub role: (),
    #[serde(rename="ServiceName")]
    pub service_name: (),
    #[serde(rename="TaskDefinition")]
    pub task_definition: (),
}

impl<'a> ::Resource<'a> for Service {
    type Properties = ServiceProperties;
    const TYPE: &'static str = "AWS::ECS::Service";
    fn properties(&self) -> &ServiceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceProperties {
        &mut self.properties
    }
}

impl From<ServiceProperties> for Service {
    fn from(properties: ServiceProperties) -> Service {
        Service { properties }
    }
}

/// The [`AWS::ECS::TaskDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html) resource.
#[derive(Serialize, Deserialize)]
pub struct TaskDefinition {
    properties: TaskDefinitionProperties
}

/// Properties for the `TaskDefinition` resource.
#[derive(Serialize, Deserialize)]
pub struct TaskDefinitionProperties {
    #[serde(rename="ContainerDefinitions")]
    pub container_definitions: (),
    #[serde(rename="Cpu")]
    pub cpu: (),
    #[serde(rename="ExecutionRoleArn")]
    pub execution_role_arn: (),
    #[serde(rename="Family")]
    pub family: (),
    #[serde(rename="Memory")]
    pub memory: (),
    #[serde(rename="NetworkMode")]
    pub network_mode: (),
    #[serde(rename="PlacementConstraints")]
    pub placement_constraints: (),
    #[serde(rename="RequiresCompatibilities")]
    pub requires_compatibilities: (),
    #[serde(rename="TaskRoleArn")]
    pub task_role_arn: (),
    #[serde(rename="Volumes")]
    pub volumes: (),
}

impl<'a> ::Resource<'a> for TaskDefinition {
    type Properties = TaskDefinitionProperties;
    const TYPE: &'static str = "AWS::ECS::TaskDefinition";
    fn properties(&self) -> &TaskDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TaskDefinitionProperties {
        &mut self.properties
    }
}

impl From<TaskDefinitionProperties> for TaskDefinition {
    fn from(properties: TaskDefinitionProperties) -> TaskDefinition {
        TaskDefinition { properties }
    }
}

