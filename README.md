# project-manager

## 導入

プロジェクトを管理するソフトウェア。モノづくりなら何でも使えるようにする。ソフトウェア開発に限定しない。目的・目標を達成するためのツール。

コンソール、GUI、WEBで使用できるクロスプラットフォームなプロジェクト管理アプリケーション。

## コンソール

指定したディレクトリにプロジェクトフォルダを作成する。
作成したフォルダ内にはアプリから操作できる「プロジェクト情報」ファイルを作成する。

プロジェクト情報ファイルには"プロジェクト作成日"、"プロジェクト名"、"最終更新日"、"プロジェクト概要"、"タグ"、"プロジェクト状態"などを設定できる。

どの情報を付与するかはアプリのセッティングから変えられる。
アプリのおおまかな機能としては「プロジェクト作成」「プロジェクト一覧」「プロジェクト情報表示」「プロジェクト状態変更」など。

プロジェクト状態は「todo」「in progress」「on hold」「done」をデフォルトとする。

プロジェクト状態のデフォルトや項目名はセッティングから変更できる。
セッティング情報はアプリ本体と同階層に設定ファイルを作成してそれを参照する。

## 概要1

### どんな機能が必要か

- ファイル類を管理する機能。
- ファイル類の変更記録を残す機能。
- タスク管理。
- スケジュール管理。
- ipad等のタブレットアプリと連携、手書きで書いたものを直接アップロードできる。
- 複数人で会議を行うときに個人メモなどのプライベート用紙と、上にスワイプして公開、共有できるパブリック用紙。
  会議で使った用紙やテキストなどは記録として残る。

タスクやスケジュールは、視点を最終目標へ固定する。今いるところではない。
最終目標が中心に来るようにする。最初に表示されるようにする。
そこから分岐して周りに伸びていく。細かいタスク、必要な機能。
終わったらノードが暗い色になる。

各ノードは、さらにそのノードを成り立たせるノードを内包する。
「最終目標」という名のノードの中に、「何を作りたいのか」「いつまでに作るのか」
「誰に向けて作るのか」などのノードがある。
同じ深さのノードはリスト。

## 概要2

タスクとそのタスクを完遂するためのステップを設定して登録。
登録されたタスクとステップは最初のステップから完遂目標のタスク方向に矢印でつなげて表示する。

それぞれのノードにはタイトル以外に作成日、完遂予定日、成果物格納、
等を設定できる。

タスクが親、各ステップがそのタスクの子という関係を持つ。
タスクとステップにもバージョン管理的な機能を持たせる。

完遂目標の変更、完遂予定日の変更、ステップの追加・削除・変更の記録など。
一度完遂したタスクは、そこから次のレベルに行きたい（目標数値が10から20）とか、そのタスクを前提とする次のタスクを、レベル2、フェーズ2として作成できる。

概要1は、最終目標となる長期的で抽象的なタスクを中心とする。
複数人で進める大規模なプロジェクトや夢、人生の目標などに使う。

こちらの概要2は、「現在」から最も近い「最初の目標」が中心。
個人もしくは小規模なグループで使用することを想定する。
現実的な短期のタスクを細かく設定してそこに至るために必要なステップを
設定する。概要1の各タスクの一部となるイメージ。
学習目標の設定や記録、進捗確認などにも使える。

## 概要3

タスク、プロジェクト、ステップを生成するときに質問形式のような形で入力を得る。  
「ブレインストーミング（メモ的なテキストボックス）」  
「あなたの最終目標は何？」  
「前提条件は？」  
「必要なものは？」  
「必要な期間は？」  
「いつまでに終わらせる？」  
「必要なタスクは？」  
「実行する順番は？」  

## 概要4

タブとエンターで簡単にノードツリー作れるようなUIにする

深さ0   1   2
プロジェクト1 Enter↓
タブ→ ステップ１ Enter↓
タブ→タブ→ タスク1 Enter↓
プロジェクト2

ctrl + enterまたはダブルクリックでそのノードを選択、詳細を表示。
詳細には概要テキスト、期日、添付ファイル一覧等を記入できる。

## 設計

### DB設計

- プロジェクトマスタ
  - ID
  - 名前
  - 作成日
  - 作成者
  - 更新日
  - 更新者

- プロジェクト詳細マスタ
  - プロジェクトID

- ノードマスタ
  - ID
  - 名前
  - ノード種別（プロジェクト、ステップ、タスク？）
  - 親ノードID
  - 作成日
  - 作成者
  - 更新日
  - 更新者
  - 期日
  - 状態
  - 添付ファイルID



