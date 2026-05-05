# Explorer/Advanced

`TaskbarStateLastRun`を除き、全てREG_DWORD

## ファイル・フォルダの表示関連

- `Hidden`: 隠しファイル・フォルダ
  - 1: 表示
  - 2: 非表示
- `ShowSuperHidden`: システムファイル(超隠しファイル)
  - 0: 非表示
  - 1: 表示
- `HideFileExt`: ファイルの拡張子
  - 0: 表示
  - 1: 非表示
- `ShowCompColor`: 圧縮・暗号化ファイル
  - 0: 色付け無し
  - 1: 色付け表示
- `IconsOnly`
  - 0: サムネイルとアイコンを表示
  - 1: アイコンのみ表示
- `ShowTypeOverlay`: ファイルの種類を示すオーバーレイアイコン 
  - 0: 非表示
  - 1: 表示
- `ShowInfoTip`: ファイルにカーソルを当てたときのツールチップ
  - 0: 非表示
  - 1: 表示 
- `HideIcons`: デスクトップアイコン
  - 0: 表示
  - 1: 非表示
- `ShowStatusBar`: エクスプローラー下部のステータスバー
  - 0: 非表示
  - 1: 表示

## タスクバー関連

- `TaskbarAnimations`: タスクバーのアニメーション
  - 0: 無効
  - 1: 有効
- `TaskbarSmallIcons`: タスクバーのアイコンサイズ
  - 0: 通常サイズ
  - 1: 小さいアイコン
- `TaskbarSizeMove`: タスクバーの移動・サイズ変更
  - 0: 禁止
  - 1: 許可
- `TaskbarAutoHideInTabletMode`: タブレットモード時の自動非表示
  - 0: 自動非表示しない
  - 1: 自動非表示
- `DisablePreviewDesktop`: タスクバーホバー時のデスクトッププレビュー (Aero Peek)
  - 0: 有効
  - 1: 無効
- `TaskbarAl`: タスクバーのアイコン配置 (Windows 11)
  - 0: 左揃え
  - 1: 中央揃え
- `TaskbarStateLastRun`: タスクバーの最終実行状態を示す内部タイムスタンプ (hex)

## スタートメニュー関連

- `Start_SearchFiles`: スタートメニューの検索対象
  - 0: ファイル検索しない
  - 1: リンクのみ検索
  - 2: ファイルも検索
- `DisablePreviewDesktop`: (タスクバー関連と共通)
- `StartMenuInit`: スタートメニュー初期化状態 (内部フラグ)
- `StartShownOnUpgrade`: アップグレード後のスタートメニュー表示済みフラグ (内部フラグ)
- `ProgrammableTaskbarStatus`: プログラムからのタスクバー制御状態 (内部フラグ)

## エクスプローラーの動作関連

- `SeparateProcess`: エクスプローラーを別プロセスで起動
  - 0: しない
  - 1: する
- `MapNetDrvBtn`: ツールバーの「ネットワークドライブの割り当て」ボタン
  - 0: 非表示
  - 1: 表示
- `DontPrettyPath`: アドレスバーのパスを整形表示 (例: `c:\windows` → `C:\Windows`)
  - 0: 整形する
  - 1: 整形しない
- `AutoCheckSelect`: チェックボックスによるファイル選択
  - 0: 無効
  - 1: 有効
- `ServerAdminUI`: サーバー管理者向けUI
  - 0: 使用しない
  - 1: 使用する
- `WebView`: エクスプローラーのWebView
  - 0: 無効
  - 1: 有効
- `Filter`: 詳細不明 (内部フラグ)

## 視覚効果関連

- `ListviewAlphaSelect`: リストビュー選択時の半透明ハイライト
  - 0: 無効
  - 1: 有効
- `ListviewShadow`: アイコンラベルの影
  - 0: 無効
  - 1: 有効

## 内部フラグ (移行・バージョン管理)

- `ShellMigrationLevel`: シェル設定の移行完了状態
- `WinXMigrationLevel`: Windows 10移行処理の完了状態
- `ReindexedProfile`: プロファイルの再インデックス完了フラグ
