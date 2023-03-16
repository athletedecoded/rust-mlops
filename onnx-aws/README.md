# AWS Lambda + EFS + Onnx Serverless Inference

## Useage

**Configure AWS IAM Role**

1. Create an AWS IAM User policy with `AWSLambda_FullAccess` permissions and custom permission config
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
2. Create your ~/.aws/credentials file with environment variables: `aws_access_key_id`, `aws_secret_access_key`, `aws_role_arn`. 

*Note: `aws_role_arn` is copied from the IAM user summary and is formatted as arn:aws:iam::<aws_acct>:user/<iam_user>*

**Install Cargo Lambda into Virtual Env**

```
$ cd onnx-aws
$ python3 -m venv ~/.venv
$ source ~/.venv/bin/activate
```

**Build Binary**

```
$ make release
```

**Deploy Lambda Fxn to AWS**
```
$ make deploy
```