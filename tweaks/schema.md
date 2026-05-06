# Tweak File Schema Definition

> **Version:** 1.0.0
> **Format:** TOML
> **Support OS** Windows 11 Home || Pro

## 概要

各カテゴリごとに1つの `.toml` ファイルを作成します。

ファイルは `[meta]` セクション（カテゴリのメタ情報）と、`[[tweaks]]` の配列（個別の設定項目）で構成されます。

- privacy.toml      # プライバシー・テレメトリ
- performance.toml  # 起動・メモリ・描画
- ui.toml           # 外観・アニメーション・タスクバー
- network.toml      # DNS・ファイアウォール・共有
- security.toml     # UAC・BitLocker・認証
- explorer.toml     # エクスプローラー・右クリックメニュー
- power.toml        # 電源・スリープ・省エネ
- input.toml        # キーボード・マウス・タッチ

## ファイル全体構造

```toml
[meta]
# カテゴリのメタ情報（1ファイルにつき1つ）

[[tweaks]]
# 設定項目（1ファイルに複数定義可）

[[tweaks]]
# ...
```

## `[meta]` セクション

カテゴリ全体のメタ情報を定義します。

|フィールド|型|必須|説明|
|---|---|:---:|---|
|`label`|`string`|✅|UIに表示するカテゴリ名|
|`description`|`string`|✅|カテゴリの概要説明|
|`icon`|`string`|✅|UIアイコン識別子（後述の定数一覧を参照）|
|`order`|`integer`|✅|サイドバーでの表示順（昇順、1から開始）|

### `[meta]`セクション例

```toml
[meta]
label       = "エクスプローラー"
description = "ファイル表示・右クリックメニュー・フォルダ動作をカスタマイズします"
icon        = "folder"
order       = 1
```

### `icon` 定数一覧

|値|用途例|
|---|---|
|`shield`|プライバシー・セキュリティ|
|`bolt`|パフォーマンス|
|`eye`|外観・UI|
|`wifi`|ネットワーク|
|`lock`|セキュリティ|
|`folder`|エクスプローラー|
|`battery`|電源・省エネ|
|`keyboard`|入力デバイス|

> アイコン識別子はアプリ側の実装に依存します。未知の値が指定された場合はデフォルトアイコンにフォールバックします。

## `[[tweaks]]` セクション

個別の設定項目を定義します。1ファイルに複数記述できます。

|フィールド|型|必須|説明|
|---|---|:---:|---|
|`id`|`string`|✅|項目の一意識別子（不変・命名規則は後述）|
|`schema_version`|`integer`|✅|スキーマバージョン（現在は`1`を指定）|
|`label`|`string`|✅|UIに表示する設定名|
|`description`|`string`|✅|設定の詳細説明 **エディションで差異がある場合はここに明示**|
|`tags`|`arrayofstring`|✅|検索用タグ（後述の推奨タグを参照）|
|`risk`|`string`|✅|リスクレベル（`"low"`/`"medium"`/`"high"`）|
|`requires_reboot`|`boolean`|✅|適用後に再起動が必要か|
|`requires_admin`|`boolean`|✅|管理者権限（HKLM書き込み等）が必要か|
|`docs_url`|`string`|❌|参考ドキュメントのURL|
|`operations`|`arrayofOperation`|✅|実際に操作するRegistryキーの定義（後述）|

### `id` 命名規則

```plaintext
<動詞>_<対象>
例: disable_telemetry
    enable_dark_mode
    set_dns_over_https
```

- 使用文字：英小文字・数字・アンダースコアのみ
- **一度公開した `id` は変更禁止**（ユーザーの適用履歴と紐づくため）
- `label` / `description` は自由に変更可

### `risk` レベルの定義

|値|意味|
|---|---|
|`"low"`|公式推奨または広く知られた安全な設定変更|
|`"medium"`|一部機能に影響する可能性があるが、復元容易|
|`"high"`|システム動作に大きく影響する。上級者向け|

### `[[tweaks]]`セクション例

```toml
[[tweaks]]
id             = "disable_telemetry"
schema_version = 1
label          = "テレメトリを無効化"
description    = "Microsoftへの診断データ・使用状況の送信を停止します。"
tags           = ["privacy", "network", "microsoft", "telemetry"]
risk           = "low"
requires_reboot = false
requires_admin  = false
docs_url       = "https://learn.microsoft.com/en-us/windows/privacy/"
operations     = [
  { hive = "HKCU", path = "Software\\Microsoft\\Windows\\CurrentVersion\\Privacy", name = "TailoredExperiencesWithDiagnosticDataEnabled", value_type = "DWORD", value_enabled = 0, value_disabled = 1 },
  { hive = "HKCU", path = "Software\\Microsoft\\InputPersonalization",             name = "RestrictImplicitInkCollection",                value_type = "DWORD", value_enabled = 1, value_disabled = 0 },
]
```

## `Operation` オブジェクト（`operations` の要素）

|フィールド|型|必須|説明|
|---|---|:---:|---|
|`hive`|`string`|✅|レジストリハイブ（後述の定数一覧を参照）|
|`path`|`string`|✅|ハイブ直下のキーパス（`\\`でエスケープ）|
|`name`|`string`|✅|値の名前|
|`value_type`|`string`|✅|値の型（後述の定数一覧を参照）|
|`value_enabled`|`integer/string`|✅|Tweakが「有効」のときに書き込む値|
|`value_disabled`|`integer/string`|✅|Tweakが「無効」（デフォルト）のときの値|

### `hive` 定数一覧

|値|正式名称|
|---|---|
|`"HKCU"`|`HKEY_CURRENT_USER`|
|`"HKLM"`|`HKEY_LOCAL_MACHINE`|
|`"HKCR"`|`HKEY_CLASSES_ROOT`|
|`"HKU"`|`HKEY_USERS`|

> `"HKLM"` を使用する場合は、親の `[[tweaks]]` で `requires_admin = true` を必ず指定してください。

### `value_type` 定数一覧

|値|Registryの型|`value_enabled/disabled`の型|
|---|---|---|
|`"DWORD"`|`REG_DWORD`（32bit整数）|`integer`|
|`"QWORD"`|`REG_QWORD`（64bit整数）|`integer`|
|`"SZ"`|`REG_SZ`（文字列）|`string`|

## バリデーションルール

アプリ起動時に以下を検証します。違反があれば起動を中止してエラーを表示します。

1. **`id` の一意性** — 全ファイルを横断して重複がないこと
2. **`hive` と `requires_admin` の整合性** — `HKLM` を使う `operation` がある場合、`requires_admin = true` であること
3. **`value_type` と値の型整合性** — `DWORD`/`QWORD` のとき `value_enabled` / `value_disabled` が整数であること
4. **`schema_version`** — アプリがサポートするバージョン以外は警告・スキップ
5. **未知フィールドの禁止** — typo検出のため、定義外フィールドはエラー

## コピー用

```toml
[meta]
label =　      ""
description = ""
icon =        ""
order =       1
```

```toml
[[tweaks]]
id              = ""
schema_version  = 1
label           = ""
description     = ""
tags            = ["", ""]
risk            = ""
requires_reboot = false
requires_admin  = false
docs_url        = ""
operations      = [
  { hive = "", path = "", name = "", value_type = "", value_enabled = 0, value_disabled = 1 },
]
```

## 変更履歴

|バージョン|変更内容|
|---|---|
|1.0.0|初版|
