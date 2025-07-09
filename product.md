# 🧾 DynamoDB Manager プロジェクト概要

## 📁 プロジェクト構造

```

dynamodb-manager/
├── LICENSE                    # MITライセンス
├── README.md                  # プロジェクトの基本情報（1行）
├── product.md                 # プロダクト設計・構成詳細
├── .env                       # AWS CLI / Terraform 用環境変数ファイル
├── aws.sh                     # AWS CLI 実行用ラッパースクリプト（Docker）
├── tf.sh                      # Terraform 実行用ラッパースクリプト（Docker）
├── new-rust.sh                # Rustプロジェクト自動生成・デプロイスクリプト
└── terraform/                 # Terraformインフラ構成
├── main.tf                # Terraformプロバイダー設定
├── variables.tf           # 変数定義（region/account\_id）
├── locals.tf              # 共通タグ定義
├── dynamodb.tf            # DynamoDBテーブル定義
├── ecr.tf                 # ECRリポジトリ定義
├── s3.tf                  # S3バケット定義
├── lambda.tf              # Lambda関数定義（ECRベース）
└── cloudfront.tf          # CloudFrontディストリビューション定義

```

---

## 🎯 プロジェクトの目的

DynamoDBのテーブル構造を**可視化・管理**し、チームで運用知識を共有するためのWebアプリケーションです。

### 主な機能
- DynamoDBテーブル/属性の一覧取得
- 各属性へのメモ（用途、注意点）の記録・編集
- サンプルデータの表示
- 属性の型を自動推定して表示
- 管理用メタ情報の永続化

---

## 🏗️ システム構成

| コンポーネント     | 技術                      | 役割                                   |
|------------------|---------------------------|----------------------------------------|
| **バックエンド**  | Rust + `lambda_http`      | APIエンドポイント（Lambda）           |
| **フロントエンド**| HTML/CSS/JavaScript       | UIインターフェース（S3にデプロイ）    |
| **データベース**  | DynamoDB                  | 実データ＋管理用メモ情報の保存         |
| **ホスティング**  | S3 + CloudFront           | 静的Webアプリの公開                    |
| **コンテナ**      | Docker + ECR              | Lambda用Rustアプリイメージのビルド     |

---

## 📄 ファイル詳細

### 📋 基本ファイル

- **`README.md`**: 簡易説明 → `DynamoDBにメモができる管理UI`
- **`LICENSE`**: MITライセンス（©️ 2025 tororoMeshi）
- **`product.md`**: アーキテクチャ・画面構成・ユースケースのドキュメント化

---

### 🔧 開発・デプロイ関連

- **`.env`**: AWS認証情報やリージョンを定義
- **`aws.sh` / `tf.sh`**: Docker上でAWS CLI/Terraformを実行（環境変数付き）
- **`new-rust.sh`**:
  - Rust Lambdaプロジェクトのテンプレート生成
  - `musl`コンパイル + セキュリティチェック + lint
  - イメージのビルド・ECRプッシュ・デプロイ補助も可能

---

### ☁️ インフラ構成（Terraform）

#### `main.tf`
- AWS Provider v5 系
- Terraform 1.5.0以降

#### `variables.tf`
- `region`, `account_id`, `environment` の定義

#### `locals.tf`
- 環境ごとのタグ定義（例：Project, Owner など）

#### `dynamodb.tf`
- **Notesテーブル**: メモ付きの属性管理
- ハッシュキー `id`（文字列）
- プロビジョンドモード（1 rcu/wcu）

#### `ecr.tf`
- `dynamodb-admin-backend` リポジトリ
- `IMMUTABLE` なタグ設定
- AES256暗号化＋ライフサイクルポリシー（古いイメージ削除）

#### `s3.tf`
- `dynamodb-admin-ui-${account_id}` というバケット名
- 静的ウェブサイトホスティング (`index.html`, `error.html`)

#### `lambda.tf`
- ECRコンテナイメージを使用した Lambda 関数
- `lambda_http` ベースのRustアプリを実行

#### `cloudfront.tf`
- S3をオリジンとするディストリビューション定義
- HTTPS + CORS対応
- オリジンアクセスコントロール（OAC）設定

---

## 🚀 デプロイフロー

1. **インフラ構築**: `terraform apply` でECR/S3/DynamoDB/Lambdaを構築
2. **アプリ構築**: Rust LambdaアプリをDocker + muslでビルド
3. **ECRへpush**: Lambda用コンテナをECRに送信
4. **Lambda更新**: 関数のイメージURIを更新
5. **UI配信**: `index.html` などを S3 に `aws s3 sync` でアップロード
6. **CDN経由で公開**: CloudFrontから https でUIへアクセス可能に

---

## 🔐 セキュリティとアクセス制御

### IAMロール要件（Lambda実行用）

- DynamoDB: `GetItem`, `PutItem`, `ListTables`, `Scan`
- CloudWatch Logs: `CreateLogGroup`, `CreateLogStream`, `PutLogEvents`
- ECR: `BatchGetImage`, `GetDownloadUrlForLayer`, `GetAuthorizationToken`

### CORS設定（Lambda + CloudFront）

Lambdaのレスポンスに以下を追加：

```http
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, POST, OPTIONS
Access-Control-Allow-Headers: Content-Type
```

CloudFrontには `OPTIONS` メソッドと `Origin` ヘッダーをキャッシュしない設定が必要。

---

## 📊 技術選定理由

| 技術                  | 採用理由                        |
| ------------------- | --------------------------- |
| **Rust**            | 高速処理・メモリ安全・Lambda向け軽量バイナリ生成 |
| **DynamoDB**        | フルマネージド・低レイテンシ・JSON互換性      |
| **S3 + CloudFront** | スケーラブルな静的配信・CDN・HTTPS標準     |
| **Terraform**       | インフラの構成管理・再現性・Git管理との親和性    |
| **Docker + ECR**    | Rust Lambdaデプロイを自動化・複数環境対応  |

---

## 🔮 今後の拡張予定

* 属性の使用頻度や参照回数の可視化
* メモ履歴（履歴差分と復元機能）
* 属性の状態（Active / Deprecated / Legacy）
* Markdownスキーマの自動生成
* 管理項目のセクション化・ソート機能

---

このプロジェクトは、DynamoDBの構造を理解・記録・改善していくための、**実践的で軽量な管理ツール**を目指しています。