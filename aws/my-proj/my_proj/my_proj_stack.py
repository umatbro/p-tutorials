from aws_cdk import core as cdk
import aws_cdk.aws_dynamodb as dynamodb
import aws_cdk.aws_batch as batch

# For consistency with other languages, `cdk` is the preferred import name for
# the CDK's core module.  The following line also imports it as `core` for use
# with examples from the CDK Developer's Guide, which are in the process of
# being updated to use `cdk`.  You may delete this import if you don't need it.
from aws_cdk import core
from aws_cdk.aws_ec2 import InstanceType, Vpc
from aws_cdk.aws_ecr import Repository, LifecycleRule
from aws_cdk.aws_ecs import ContainerImage
from aws_cdk.aws_iam import Role, ServicePrincipal, ManagedPolicy


class MyProjStack(cdk.Stack):
    def __init__(self, scope: cdk.Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        # The code that defines your stack goes here
        table = dynamodb.Table(
            self,
            "TheTable",
            table_name="cdk-table",
            partition_key=dynamodb.Attribute(
                name="id", type=dynamodb.AttributeType.STRING
            ),
            removal_policy=cdk.RemovalPolicy.DESTROY,
        )

        # compute_environment = batch.ComputeEnvironment(
        #     self,
        #     "MyComputeEnvironment",
        #     compute_environment_name="cdk-env",
        #     compute_resources=batch.ComputeResources(
        #         vpc=Vpc.from_lookup(self, "VPC", is_default=True),
        #     ),
        #     enabled=True,
        #     managed=True,
        # )

        job_role = Role(
            self,
            "BatchJobRole",
            assumed_by=ServicePrincipal("ecs-tasks.amazonaws.com"),
            description="Role for a container in a Batch job",
            role_name="CDK-BatchJobRole",
            managed_policies=[
                ManagedPolicy.from_aws_managed_policy_name(
                    managed_policy_name="AmazonDynamoDBFullAccess"
                ),
            ],
        )

        repository = Repository(
            self,
            "MyRepository",
            removal_policy=cdk.RemovalPolicy.DESTROY,
            repository_name="cdk-my-repository",
            lifecycle_rules=[
                LifecycleRule(max_image_count=5, description="Max 5 images")
            ],
        )

        image: ContainerImage = ContainerImage.from_ecr_repository(
            repository=repository,
            tag="latest",
        )

        container = batch.JobDefinitionContainer(
            image=image,
            job_role=job_role,
            command=["python", "run.py", "--help"],
            environment={
                "READINGS_TABLE": table.table_name,
                "AWS_REGION": self.region,
            },
            vcpus=1,
            log_configuration=batch.LogConfiguration(
                log_driver=batch.LogDriver.AWSLOGS
            ),
            memory_limit_mib=2048,
        )

        batch.JobDefinition(
            self,
            "JobDefinitionCreate",
            container=container,
            job_definition_name="create",
            retry_attempts=1,
        )
