import { LambdaIntegration, RestApi } from '@aws-cdk/aws-apigateway';
import { Code, Function, Runtime } from '@aws-cdk/aws-lambda';
import { Construct, Duration, Stack, StackProps } from '@aws-cdk/core';

export class LambdaStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props);

        const savePostFunction = new Function(
            this,
            'savePostFunction',
            {
                functionName: 'savePostFunction',
                runtime: Runtime.PROVIDED_AL2,
                handler: 'handler',
                code: Code.fromAsset(`${__dirname}/../../functions/save-post/target/cdk/debug`),
                timeout: Duration.seconds(10)
            }
        )

        const api = new RestApi(
            this,
            'restApi',
            {
                restApiName: "ytakada.dev API"
            }
        );
        const posts = api.root.addResource('posts');
        const singlePost = posts.addResource('{id}');
        const savePostIntegration = new LambdaIntegration(savePostFunction);
        singlePost.addMethod("PUT", savePostIntegration);
    }
}
