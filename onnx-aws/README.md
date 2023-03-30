# AWS Lambda + EFS + Onnx Serverless Inference

## Useage

**Configure AWS IAM Permissions**

1. Create an IAM User policy for "LambdaDev" with `AWSLambda_FullAccess` permissions and added custom inline permission config
```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "PermissionsToDeploy",
            "Effect": "Allow",
            "Action": [
                "iam:CreateRole",
                "iam:CreatePolicy",
                "iam:PutRolePolicy",
                "iam:AttachRolePolicy",
                "iam:UpdateAssumeRolePolicy"
            ],
            "Resource": "*"
        }
    ]
}
```
2. Add "LambdaDev" access keys to your local ~/.aws/credentials file along with environment variables: 
* aws_access_key_id
* aws_secret_access_key
* aws_role_arn (NB: this is copied from the IAM user summary and is formatted as arn:aws:iam::<aws_acct>:user/<iam_user>)
* region 

3. Create an IAM Role policy named "EFSxLambda" with `AWSLambdaBasicExecutionRole` + `AWSLambdaVPCAccessExecutionRole` + `AmazonElasticFileSystemClientFullAccess` permissions

**Deploy Lambda Fxn from Developer Environment i.e. this repo**

```
# Install cargo-lambda
$ cd onnx-aws
$ python3 -m venv ~/.venv
$ source ~/.venv/bin/activate

# Build Binary 
$ make release

# Deploy to AWS
$ make deploy
```

**Setup Cloud9**

1. Launch Cloud9 console and create an environment with "New EC2 Instance" + "SSM Connection" + "default VPC"
2. Once provisioned, click on env details > EC2 instance > manage EC2 instance. Make note of the associated security group listed.

**Setup EFS**

1. Launch AWS EFS console and provision EFS instance
2. Once provisioned, click on file system name > network. Make note of the security group id listed. 
3. Under EFS access points tab > add access point > create with following settings:
  * Name: LambdaEFS
  * Root directory path: `/mnt/efs`
  * Posix User ID: 1000
  * Posix Group ID: 1000
  * Owner User ID: 1000
  * Owner Group ID: 1000
  * Access point permissions: 0777

**Configure Security Groups**

1. From EC2 console > security groups > new security group: 
  * Name = onnxLambda
  * Description = Mangage onnx-aws lambda access
  * VPC = default/same as EFS
2. To enable EFS to communicate with Lambda, from EC2 console > security groups > click onnxLambda security group ID > edit inbound rules > Set: type = NFS, protocol = TCP, port range = 2049, source = custom & select the EFS security group ID (from the drop down box).
3. To enable Lambda and Cloud9 to communicate with EFS, from EC2 console > security groups > click EFS security group ID > edit inbound rules > add 2 inbound rules for each:
  * Set: type = NFS, protocol = TCP, port range = 2049, source = custom & select the EFS security group ID (from the drop down box)
  * Set: type = NFS, protocol = TCP, port range = 2049, source = custom & select the Cloud9 security group ID (from the drop down box)

**Configure Lambda for EFS**

1. Configuration > Permissions > edit execution role > select "EFSxLambda" from above
2. Configuration > VPC > select default VPC (to match EFS)
3. Configuration > VPC > edit security group to EFSxLambda
4. Configuration > File Systems > add EFS + access point + local mount path = /mnt/efs
5. Configuration > Env Variables > LD_LIBRARY_PATH = /mnt/efs


**Mount EFS to Cloud9**

1. Launch Cloud9 environment
2. Run the following commands to mount to /mnt/efs

```
# Install efs-ultils (https://docs.aws.amazon.com/efs/latest/ug/installing-amazon-efs-utils.html)
sudo yum install -y amazon-efs-utils

sudo mkdir /mnt/efs

# From EFS "Attach" copy EFS helper command
sudo mount -t efs -o tls <AWS_FS_ID>:/ /mnt/efs
cd /mnt/efs

# Switch ownership permissions
sudo chown ec2-user:ec2-user /mnt/efs
sudo chmod 755 /mnt/efs

# Test
touch foo.txt
ls

# Download squeezenet model
wget https://github.com/onnx/models/blob/main/vision/classification/squeezenet/model/squeezenet1.0-12.onnx?raw=true -O squeezenet1.0-12.onnx

```







## References
* [AWS EFS + Lambda Guide](https://aws.amazon.com/blogs/compute/using-amazon-efs-for-aws-lambda-in-your-serverless-applications/)
* [Mount EC2 to EFS](https://docs.aws.amazon.com/efs/latest/ug/mounting-fs-mount-helper-ec2-linux.html)