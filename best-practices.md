# 1. TerraformでECR作成
cd terraform
./tf.sh init
./tf.sh plan
./tf.sh apply

# 2. アプリビルドしてECRにPush
cd ../my-lambda
./push_to_ecr.sh
