# 🧾 DynamoDB Manager プロジェクト概要

## 📁 プロジェクト構造

```
dynamodb-manager/
├── LICENSE                    # MITライセンス
├── README.md                  # プロジェクトの基本情報
├── product.md                 # プロダクト設計・構成詳細
├── new-rust.sh               # Rustプロジェクト自動生成スクリプト
└── terraform/                # Terraformインフラ構成
    ├── main.tf               # Terraformプロバイダー設定
    ├── variables.tf          # 変数定義
    ├── locals.tf             # ローカル変数・タグ設定
    ├── dynamodb.tf           # DynamoDBテーブル定義
    ├── ecr.tf                # ECRリポジトリ定義
    └── s3.tf                 # S3バケット定義
```

## 🎯 プロジェクトの目的

DynamoDBのテーブル構造を**可視化・管理**し、チームで運用知識を共有するためのWebアプリケーションです。

### 主な機能
- DynamoDBテーブル一覧・属性一覧の取得
- 各属性に対するメモ（用途、注意点など）の記録・編集
- サンプルデータの表示
- 属性の自動型推定機能

## 🏗️ システム構成

| コンポーネント | 技術 | 役割 |
|----------------|------|------|
| **バックエンド** | Rust + lambda_http | APIエンドポイント（Lambda） |
| **フロントエンド** | HTML/CSS/JavaScript | UIインターフェース |
| **データベース** | DynamoDB | 実データ + メタ情報管理 |
| **ホスティング** | S3 + CloudFront | 静的サイト配信 |
| **コンテナ** | Docker + ECR | Lambda用コンテナイメージ |

## 📄 ファイル詳細

### 📋 基本ファイル

- **`README.md`**: プロジェクトの基本情報（1行のみ）
- **`LICENSE`**: MITライセンス（2025 tororoMeshi）
- **`product.md`**: プロダクト設計・アーキテクチャの詳細説明

### 🔧 開発・デプロイ関連

- **`new-rust.sh`**: Rustプロジェクト自動生成スクリプト
  - Dockerを使用したRustプロジェクトのテンプレート生成
  - ECRプッシュ、リント、セキュリティチェック機能
  - Terraform連携スクリプト付き

### ☁️ インフラ構成（Terraform）

#### `terraform/main.tf`
- AWS Provider設定（v5.0系）
- Terraform 1.5.0以降対応

#### `terraform/variables.tf`
- `region`: AWSリージョン
- `account_id`: AWSアカウントID
- `environment`: 環境名（デフォルト: prod）

#### `terraform/locals.tf`
- 共通タグ設定
- プロジェクト固有の設定値

#### `terraform/dynamodb.tf`
- **Notesテーブル**: メモ管理用DynamoDBテーブル
- プロビジョニングモード（read/write capacity: 1）
- ハッシュキー: `id` (String)

#### `terraform/ecr.tf`
- **dynamodb-admin-backend**: ECRリポジトリ
- イメージタグ不変性設定
- AES256暗号化
- ライフサイクルポリシー（14日後に未タグ削除、10個以上のイメージ削除）

#### `terraform/s3.tf`
- **フロントエンド用S3バケット**: `dynamodb-admin-ui-{account_id}`
- 静的ウェブサイトホスティング設定
- インデックス・エラードキュメント: `index.html`

## 🚀 デプロイフロー

1. **インフラ準備**: Terraformで AWS リソース作成
2. **アプリケーションビルド**: Rustアプリをコンテナ化
3. **ECRプッシュ**: コンテナイメージをECRに送信
4. **Lambda設定**: ECRイメージを使用したLambda関数作成
5. **フロントエンドアップロード**: S3に静的ファイルをアップロード
6. **CloudFront設定**: HTTPS配信設定

## 🔐 セキュリティ・権限

### IAMロール要件（Lambda実行用）
- DynamoDB: `GetItem`, `PutItem`, `ListTables`, `Scan`
- Logs: `CreateLogGroup`, `PutLogEvents`
- ECR: `GetAuthorizationToken`, `GetDownloadUrlForLayer`

### CORS対応
- Lambda APIでCORSヘッダー設定
- CloudFrontでのキャッシュ制御

## 📊 技術選定理由

- **Rust**: 高速性能、メモリ安全性、Lambda最適化
- **DynamoDB**: サーバーレス、自動スケーリング、AWSネイティブ
- **S3 + CloudFront**: 高可用性、CDN、HTTPS対応
- **Terraform**: インフラコード化、バージョン管理、再現性

## 🎯 今後の拡張予定

- 属性利用頻度の表示機能
- メモ変更履歴・バージョン管理
- 属性ステータス管理（Active/Deprecated）
- Markdownスキーマドキュメント自動生成
- 管理メタのグループ化・並び替え機能

---

このプロジェクトは、DynamoDBの運用知識を効率的に管理・共有するためのシンプルかつ実用的なツールです。