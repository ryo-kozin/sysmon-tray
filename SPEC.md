# System Monitor - 仕様書

## 概要

軽量・クロスプラットフォーム対応のシステムリソース監視ツール。
メニューバー/システムトレイに常駐し、リソース使用状況の表示と閾値超過時の通知を行う。

## 技術スタック

- **フレームワーク**: Tauri v2
- **バックエンド**: Rust（sysinfo crate）
- **フロントエンド**: React + TypeScript
- **対応OS**: macOS / Windows / Linux

## 機能

### MVP（v0.1）

#### メニューバー/トレイ表示
- CPU使用率（リアルタイム）
- メモリ使用率
- ディスク残量
- 更新間隔: 3秒（設定可能）

#### 通知
- CPU: 閾値超過時に通知（デフォルト80%、10秒以上継続）
- メモリ: 空き割合が閾値未満で通知（デフォルト10%）
- ディスク: 残量が閾値未満で通知（デフォルト10GB）
- クールダウン: 同一通知の再送間隔（デフォルト15分）
- 通知にはトリガーしたプロセス名を含める

#### 設定
- 各閾値のカスタマイズ
- 通知ON/OFF（項目別）
- 更新間隔の変更
- ログイン時自動起動

### v0.2

- クリックで詳細パネル表示（プロセス一覧、CPU/メモリソート）
- プロセスのkill機能
- 通知履歴

### v0.3

- ネットワーク帯域監視
- 温度/ファン回転数（macOS/Linux）
- バッテリー情報（ノートPC）
- テーマ（ライト/ダーク、OS連動）

### 将来検討

- Webhook/Slack通知連携
- リソース使用履歴のグラフ表示
- プラグインシステム
- 多言語対応（日本語/英語）

## 非機能要件

- メモリ使用量: 30MB以下（Electronの1/5を目標）
- CPU使用率: アイドル時0.5%以下
- バイナリサイズ: 15MB以下
- OS通知API使用（macOS: UNUserNotificationCenter, Windows: Toast, Linux: libnotify）

## ディレクトリ構成（予定）

```
system-monitor/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── monitor.rs  # sysinfo によるリソース取得
│   │   ├── notifier.rs # 通知ロジック
│   │   └── config.rs   # 設定管理
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                # React frontend
│   ├── App.tsx
│   ├── components/
│   │   ├── TrayView.tsx    # トレイポップアップ
│   │   ├── StatusBar.tsx   # リソースバー表示
│   │   └── Settings.tsx    # 設定画面
│   └── hooks/
│       └── useSystemInfo.ts
├── package.json
├── SPEC.md
├── LICENSE             # MIT
└── README.md
```

## 差別化ポイント

| 比較 | Stats | iStat Menus | Alertivity | 本ツール |
|------|-------|-------------|------------|---------|
| OSS | Yes | No | Yes | Yes |
| クロスプラットフォーム | No | No | No | Yes |
| 通知機能 | 弱い | 強い | あり | あり |
| 軽量 | Yes | Yes | Yes | Yes |
| 価格 | 無料 | 有料 | 無料 | 無料 |

## ライセンス

MIT
