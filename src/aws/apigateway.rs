/// The [`AWS::ApiGateway::Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-account.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Account {
    properties: AccountProperties
}

/// Properties for the `Account` resource.
#[derive(Serialize, Deserialize)]
pub struct AccountProperties {
    #[serde(rename="CloudWatchRoleArn")]
    pub cloud_watch_role_arn: (),
}

impl<'a> ::Resource<'a> for Account {
    type Properties = AccountProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Account";
    fn properties(&self) -> &AccountProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccountProperties {
        &mut self.properties
    }
}

impl From<AccountProperties> for Account {
    fn from(properties: AccountProperties) -> Account {
        Account { properties }
    }
}

/// The [`AWS::ApiGateway::ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ApiKey {
    properties: ApiKeyProperties
}

/// Properties for the `ApiKey` resource.
#[derive(Serialize, Deserialize)]
pub struct ApiKeyProperties {
    #[serde(rename="CustomerId")]
    pub customer_id: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Enabled")]
    pub enabled: (),
    #[serde(rename="GenerateDistinctId")]
    pub generate_distinct_id: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="StageKeys")]
    pub stage_keys: (),
}

impl<'a> ::Resource<'a> for ApiKey {
    type Properties = ApiKeyProperties;
    const TYPE: &'static str = "AWS::ApiGateway::ApiKey";
    fn properties(&self) -> &ApiKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApiKeyProperties {
        &mut self.properties
    }
}

impl From<ApiKeyProperties> for ApiKey {
    fn from(properties: ApiKeyProperties) -> ApiKey {
        ApiKey { properties }
    }
}

/// The [`AWS::ApiGateway::Authorizer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Authorizer {
    properties: AuthorizerProperties
}

/// Properties for the `Authorizer` resource.
#[derive(Serialize, Deserialize)]
pub struct AuthorizerProperties {
    #[serde(rename="AuthType")]
    pub auth_type: (),
    #[serde(rename="AuthorizerCredentials")]
    pub authorizer_credentials: (),
    #[serde(rename="AuthorizerResultTtlInSeconds")]
    pub authorizer_result_ttl_in_seconds: (),
    #[serde(rename="AuthorizerUri")]
    pub authorizer_uri: (),
    #[serde(rename="IdentitySource")]
    pub identity_source: (),
    #[serde(rename="IdentityValidationExpression")]
    pub identity_validation_expression: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="ProviderARNs")]
    pub provider_ar_ns: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
    #[serde(rename="Type")]
    pub type_: (),
}

impl<'a> ::Resource<'a> for Authorizer {
    type Properties = AuthorizerProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Authorizer";
    fn properties(&self) -> &AuthorizerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AuthorizerProperties {
        &mut self.properties
    }
}

impl From<AuthorizerProperties> for Authorizer {
    fn from(properties: AuthorizerProperties) -> Authorizer {
        Authorizer { properties }
    }
}

/// The [`AWS::ApiGateway::BasePathMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html) resource.
#[derive(Serialize, Deserialize)]
pub struct BasePathMapping {
    properties: BasePathMappingProperties
}

/// Properties for the `BasePathMapping` resource.
#[derive(Serialize, Deserialize)]
pub struct BasePathMappingProperties {
    #[serde(rename="BasePath")]
    pub base_path: (),
    #[serde(rename="DomainName")]
    pub domain_name: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
    #[serde(rename="Stage")]
    pub stage: (),
}

impl<'a> ::Resource<'a> for BasePathMapping {
    type Properties = BasePathMappingProperties;
    const TYPE: &'static str = "AWS::ApiGateway::BasePathMapping";
    fn properties(&self) -> &BasePathMappingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BasePathMappingProperties {
        &mut self.properties
    }
}

impl From<BasePathMappingProperties> for BasePathMapping {
    fn from(properties: BasePathMappingProperties) -> BasePathMapping {
        BasePathMapping { properties }
    }
}

/// The [`AWS::ApiGateway::ClientCertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-clientcertificate.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ClientCertificate {
    properties: ClientCertificateProperties
}

/// Properties for the `ClientCertificate` resource.
#[derive(Serialize, Deserialize)]
pub struct ClientCertificateProperties {
    #[serde(rename="Description")]
    pub description: (),
}

impl<'a> ::Resource<'a> for ClientCertificate {
    type Properties = ClientCertificateProperties;
    const TYPE: &'static str = "AWS::ApiGateway::ClientCertificate";
    fn properties(&self) -> &ClientCertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClientCertificateProperties {
        &mut self.properties
    }
}

impl From<ClientCertificateProperties> for ClientCertificate {
    fn from(properties: ClientCertificateProperties) -> ClientCertificate {
        ClientCertificate { properties }
    }
}

/// The [`AWS::ApiGateway::Deployment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Deployment {
    properties: DeploymentProperties
}

/// Properties for the `Deployment` resource.
#[derive(Serialize, Deserialize)]
pub struct DeploymentProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
    #[serde(rename="StageDescription")]
    pub stage_description: (),
    #[serde(rename="StageName")]
    pub stage_name: (),
}

impl<'a> ::Resource<'a> for Deployment {
    type Properties = DeploymentProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Deployment";
    fn properties(&self) -> &DeploymentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeploymentProperties {
        &mut self.properties
    }
}

impl From<DeploymentProperties> for Deployment {
    fn from(properties: DeploymentProperties) -> Deployment {
        Deployment { properties }
    }
}

/// The [`AWS::ApiGateway::DocumentationPart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationpart.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DocumentationPart {
    properties: DocumentationPartProperties
}

/// Properties for the `DocumentationPart` resource.
#[derive(Serialize, Deserialize)]
pub struct DocumentationPartProperties {
    #[serde(rename="Location")]
    pub location: (),
    #[serde(rename="Properties")]
    pub properties: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
}

impl<'a> ::Resource<'a> for DocumentationPart {
    type Properties = DocumentationPartProperties;
    const TYPE: &'static str = "AWS::ApiGateway::DocumentationPart";
    fn properties(&self) -> &DocumentationPartProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentationPartProperties {
        &mut self.properties
    }
}

impl From<DocumentationPartProperties> for DocumentationPart {
    fn from(properties: DocumentationPartProperties) -> DocumentationPart {
        DocumentationPart { properties }
    }
}

/// The [`AWS::ApiGateway::DocumentationVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationversion.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DocumentationVersion {
    properties: DocumentationVersionProperties
}

/// Properties for the `DocumentationVersion` resource.
#[derive(Serialize, Deserialize)]
pub struct DocumentationVersionProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="DocumentationVersion")]
    pub documentation_version: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
}

impl<'a> ::Resource<'a> for DocumentationVersion {
    type Properties = DocumentationVersionProperties;
    const TYPE: &'static str = "AWS::ApiGateway::DocumentationVersion";
    fn properties(&self) -> &DocumentationVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentationVersionProperties {
        &mut self.properties
    }
}

impl From<DocumentationVersionProperties> for DocumentationVersion {
    fn from(properties: DocumentationVersionProperties) -> DocumentationVersion {
        DocumentationVersion { properties }
    }
}

/// The [`AWS::ApiGateway::DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html) resource.
#[derive(Serialize, Deserialize)]
pub struct DomainName {
    properties: DomainNameProperties
}

/// Properties for the `DomainName` resource.
#[derive(Serialize, Deserialize)]
pub struct DomainNameProperties {
    #[serde(rename="CertificateArn")]
    pub certificate_arn: (),
    #[serde(rename="DomainName")]
    pub domain_name: (),
    #[serde(rename="EndpointConfiguration")]
    pub endpoint_configuration: (),
    #[serde(rename="RegionalCertificateArn")]
    pub regional_certificate_arn: (),
}

impl<'a> ::Resource<'a> for DomainName {
    type Properties = DomainNameProperties;
    const TYPE: &'static str = "AWS::ApiGateway::DomainName";
    fn properties(&self) -> &DomainNameProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainNameProperties {
        &mut self.properties
    }
}

impl From<DomainNameProperties> for DomainName {
    fn from(properties: DomainNameProperties) -> DomainName {
        DomainName { properties }
    }
}

/// The [`AWS::ApiGateway::GatewayResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html) resource.
#[derive(Serialize, Deserialize)]
pub struct GatewayResponse {
    properties: GatewayResponseProperties
}

/// Properties for the `GatewayResponse` resource.
#[derive(Serialize, Deserialize)]
pub struct GatewayResponseProperties {
    #[serde(rename="ResponseParameters")]
    pub response_parameters: (),
    #[serde(rename="ResponseTemplates")]
    pub response_templates: (),
    #[serde(rename="ResponseType")]
    pub response_type: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
    #[serde(rename="StatusCode")]
    pub status_code: (),
}

impl<'a> ::Resource<'a> for GatewayResponse {
    type Properties = GatewayResponseProperties;
    const TYPE: &'static str = "AWS::ApiGateway::GatewayResponse";
    fn properties(&self) -> &GatewayResponseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GatewayResponseProperties {
        &mut self.properties
    }
}

impl From<GatewayResponseProperties> for GatewayResponse {
    fn from(properties: GatewayResponseProperties) -> GatewayResponse {
        GatewayResponse { properties }
    }
}

/// The [`AWS::ApiGateway::Method`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Method {
    properties: MethodProperties
}

/// Properties for the `Method` resource.
#[derive(Serialize, Deserialize)]
pub struct MethodProperties {
    #[serde(rename="ApiKeyRequired")]
    pub api_key_required: (),
    #[serde(rename="AuthorizationType")]
    pub authorization_type: (),
    #[serde(rename="AuthorizerId")]
    pub authorizer_id: (),
    #[serde(rename="HttpMethod")]
    pub http_method: (),
    #[serde(rename="Integration")]
    pub integration: (),
    #[serde(rename="MethodResponses")]
    pub method_responses: (),
    #[serde(rename="OperationName")]
    pub operation_name: (),
    #[serde(rename="RequestModels")]
    pub request_models: (),
    #[serde(rename="RequestParameters")]
    pub request_parameters: (),
    #[serde(rename="RequestValidatorId")]
    pub request_validator_id: (),
    #[serde(rename="ResourceId")]
    pub resource_id: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
}

impl<'a> ::Resource<'a> for Method {
    type Properties = MethodProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Method";
    fn properties(&self) -> &MethodProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MethodProperties {
        &mut self.properties
    }
}

impl From<MethodProperties> for Method {
    fn from(properties: MethodProperties) -> Method {
        Method { properties }
    }
}

/// The [`AWS::ApiGateway::Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Model {
    properties: ModelProperties
}

/// Properties for the `Model` resource.
#[derive(Serialize, Deserialize)]
pub struct ModelProperties {
    #[serde(rename="ContentType")]
    pub content_type: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
    #[serde(rename="Schema")]
    pub schema: (),
}

impl<'a> ::Resource<'a> for Model {
    type Properties = ModelProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Model";
    fn properties(&self) -> &ModelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelProperties {
        &mut self.properties
    }
}

impl From<ModelProperties> for Model {
    fn from(properties: ModelProperties) -> Model {
        Model { properties }
    }
}

/// The [`AWS::ApiGateway::RequestValidator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html) resource.
#[derive(Serialize, Deserialize)]
pub struct RequestValidator {
    properties: RequestValidatorProperties
}

/// Properties for the `RequestValidator` resource.
#[derive(Serialize, Deserialize)]
pub struct RequestValidatorProperties {
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
    #[serde(rename="ValidateRequestBody")]
    pub validate_request_body: (),
    #[serde(rename="ValidateRequestParameters")]
    pub validate_request_parameters: (),
}

impl<'a> ::Resource<'a> for RequestValidator {
    type Properties = RequestValidatorProperties;
    const TYPE: &'static str = "AWS::ApiGateway::RequestValidator";
    fn properties(&self) -> &RequestValidatorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RequestValidatorProperties {
        &mut self.properties
    }
}

impl From<RequestValidatorProperties> for RequestValidator {
    fn from(properties: RequestValidatorProperties) -> RequestValidator {
        RequestValidator { properties }
    }
}

/// The [`AWS::ApiGateway::Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-resource.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Resource {
    properties: ResourceProperties
}

/// Properties for the `Resource` resource.
#[derive(Serialize, Deserialize)]
pub struct ResourceProperties {
    #[serde(rename="ParentId")]
    pub parent_id: (),
    #[serde(rename="PathPart")]
    pub path_part: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
}

impl<'a> ::Resource<'a> for Resource {
    type Properties = ResourceProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Resource";
    fn properties(&self) -> &ResourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceProperties {
        &mut self.properties
    }
}

impl From<ResourceProperties> for Resource {
    fn from(properties: ResourceProperties) -> Resource {
        Resource { properties }
    }
}

/// The [`AWS::ApiGateway::RestApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html) resource.
#[derive(Serialize, Deserialize)]
pub struct RestApi {
    properties: RestApiProperties
}

/// Properties for the `RestApi` resource.
#[derive(Serialize, Deserialize)]
pub struct RestApiProperties {
    #[serde(rename="ApiKeySourceType")]
    pub api_key_source_type: (),
    #[serde(rename="BinaryMediaTypes")]
    pub binary_media_types: (),
    #[serde(rename="Body")]
    pub body: (),
    #[serde(rename="BodyS3Location")]
    pub body_s3_location: (),
    #[serde(rename="CloneFrom")]
    pub clone_from: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="EndpointConfiguration")]
    pub endpoint_configuration: (),
    #[serde(rename="FailOnWarnings")]
    pub fail_on_warnings: (),
    #[serde(rename="MinimumCompressionSize")]
    pub minimum_compression_size: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Parameters")]
    pub parameters: (),
}

impl<'a> ::Resource<'a> for RestApi {
    type Properties = RestApiProperties;
    const TYPE: &'static str = "AWS::ApiGateway::RestApi";
    fn properties(&self) -> &RestApiProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RestApiProperties {
        &mut self.properties
    }
}

impl From<RestApiProperties> for RestApi {
    fn from(properties: RestApiProperties) -> RestApi {
        RestApi { properties }
    }
}

/// The [`AWS::ApiGateway::Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Stage {
    properties: StageProperties
}

/// Properties for the `Stage` resource.
#[derive(Serialize, Deserialize)]
pub struct StageProperties {
    #[serde(rename="CacheClusterEnabled")]
    pub cache_cluster_enabled: (),
    #[serde(rename="CacheClusterSize")]
    pub cache_cluster_size: (),
    #[serde(rename="ClientCertificateId")]
    pub client_certificate_id: (),
    #[serde(rename="DeploymentId")]
    pub deployment_id: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="DocumentationVersion")]
    pub documentation_version: (),
    #[serde(rename="MethodSettings")]
    pub method_settings: (),
    #[serde(rename="RestApiId")]
    pub rest_api_id: (),
    #[serde(rename="StageName")]
    pub stage_name: (),
    #[serde(rename="Variables")]
    pub variables: (),
}

impl<'a> ::Resource<'a> for Stage {
    type Properties = StageProperties;
    const TYPE: &'static str = "AWS::ApiGateway::Stage";
    fn properties(&self) -> &StageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StageProperties {
        &mut self.properties
    }
}

impl From<StageProperties> for Stage {
    fn from(properties: StageProperties) -> Stage {
        Stage { properties }
    }
}

/// The [`AWS::ApiGateway::UsagePlan`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html) resource.
#[derive(Serialize, Deserialize)]
pub struct UsagePlan {
    properties: UsagePlanProperties
}

/// Properties for the `UsagePlan` resource.
#[derive(Serialize, Deserialize)]
pub struct UsagePlanProperties {
    #[serde(rename="ApiStages")]
    pub api_stages: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Quota")]
    pub quota: (),
    #[serde(rename="Throttle")]
    pub throttle: (),
    #[serde(rename="UsagePlanName")]
    pub usage_plan_name: (),
}

impl<'a> ::Resource<'a> for UsagePlan {
    type Properties = UsagePlanProperties;
    const TYPE: &'static str = "AWS::ApiGateway::UsagePlan";
    fn properties(&self) -> &UsagePlanProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UsagePlanProperties {
        &mut self.properties
    }
}

impl From<UsagePlanProperties> for UsagePlan {
    fn from(properties: UsagePlanProperties) -> UsagePlan {
        UsagePlan { properties }
    }
}

/// The [`AWS::ApiGateway::UsagePlanKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplankey.html) resource.
#[derive(Serialize, Deserialize)]
pub struct UsagePlanKey {
    properties: UsagePlanKeyProperties
}

/// Properties for the `UsagePlanKey` resource.
#[derive(Serialize, Deserialize)]
pub struct UsagePlanKeyProperties {
    #[serde(rename="KeyId")]
    pub key_id: (),
    #[serde(rename="KeyType")]
    pub key_type: (),
    #[serde(rename="UsagePlanId")]
    pub usage_plan_id: (),
}

impl<'a> ::Resource<'a> for UsagePlanKey {
    type Properties = UsagePlanKeyProperties;
    const TYPE: &'static str = "AWS::ApiGateway::UsagePlanKey";
    fn properties(&self) -> &UsagePlanKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UsagePlanKeyProperties {
        &mut self.properties
    }
}

impl From<UsagePlanKeyProperties> for UsagePlanKey {
    fn from(properties: UsagePlanKeyProperties) -> UsagePlanKey {
        UsagePlanKey { properties }
    }
}

/// The [`AWS::ApiGateway::VpcLink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-vpclink.html) resource.
#[derive(Serialize, Deserialize)]
pub struct VpcLink {
    properties: VpcLinkProperties
}

/// Properties for the `VpcLink` resource.
#[derive(Serialize, Deserialize)]
pub struct VpcLinkProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="TargetArns")]
    pub target_arns: (),
}

impl<'a> ::Resource<'a> for VpcLink {
    type Properties = VpcLinkProperties;
    const TYPE: &'static str = "AWS::ApiGateway::VpcLink";
    fn properties(&self) -> &VpcLinkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VpcLinkProperties {
        &mut self.properties
    }
}

impl From<VpcLinkProperties> for VpcLink {
    fn from(properties: VpcLinkProperties) -> VpcLink {
        VpcLink { properties }
    }
}

