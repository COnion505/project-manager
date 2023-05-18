# 導入
「なんか作ろう」「ちょっとコード書こう」「いいアイディア浮かんだ」って時にパッとプロジェクトを作成・管理したいなと思い、Rustでのプログラミングの学習も兼ねて簡易的なプロジェクト管理アプリケーションを作っていきます。タイトルで（コンソール）としているのは、「将来的にGUIアプリケーション・WEBアプリケーションとしても作りたいなあ」と考えてのことです。


# 設計
## 機能
  1. プロジェクトの新規作成、ショートカットリンクの作成
  1. 作成済みプロジェクトの一覧表示
  1. プロジェクト情報の表示
  1. プロジェクト情報の編集
  1. プロジェクトを開く
  1. 環境設定の編集
  1. プログラムの終了
- プロジェクトの作成場所・ショートカットの作成場所などは環境変数で管理する。
- その他環境設定はyamlファイルで管理する。  
新規作成時に付与する情報(tags, overview, その他任意の項目)、  
ショートカットリンクの作成有無、  
情報編集で使用するエディダ指定、  
など。
- 本プログラム単体で実行した場合、上記機能の選択を対話型で選択された機能を実行する。プログラムの終了が選択されるまでループする。
- 引数で上記機能のコマンド名が渡された場合、対話型処理には入らず直接機能を実行する。
- 「プロジェクト情報の表示」で表示する情報はプロジェクト新規作成時にプロジェクトフォルダ内に生成されるプロジェクト情報ファイル(.projectinfo)を参照する。
- 「プロジェクト情報の編集」はプロジェクト情報ファイル(.projectinfo)を編集する。


## インターフェース
プロジェクトの新規作成
```console
$ manage-project create project-name
project-name is created.
project-path: 環境変数に設定されたプロジェクト作成場所/project-name
project-link: 環境変数に設定されたショートカット作成場所/project-name

$ manage-project create project-name
project-name already exists.
```

プロジェクト情報の表示
.projectinfoファイルの内容を出力する。
```console
$ manage-project info project-name
project name: project-name
created at: 2022/10/22 3:31:48
overview: 
tags: 

$ manage-project info project2
project2 not exists
```

プロジェクト情報の編集(-m: modify)  
.projectinfoファイルをvim(または設定ファイルで指定されているエディダ)で開く。
```console
$ manage-project info -m project-name

$ manage-project info -m project2
project2 not exists
```

対話型
```console
$ manage-project
>>>create project2
project2 is created.
project-path: 環境変数に設定されたプロジェクト作成場所/project2
project-link: 環境変数に設定されたショートカット作成場所/project2

>>>info project2
project name: project2
created at: 2022/10/22 18:28:50
overview: 
tags: 

```

## 処理の流れ
### 初期処理
SVC側：環境設定チェック  
SVC側：チェック結果正常 => 状態列挙  


### メイン処理
UI側：プログラム引数取得
UI側：プログラム引数解析
空白 => 対話型処理へ
create => プロジェクト新規作成処理へ
info => プロジェクト情報表示処理へ
create => プロジェクト新規作成処理へ

### プロジェクト新規作成
UI側：プロジェクト名の入力受付  
UI側：プロジェクト名をプロジェクト作成サービスへ渡す  
SVC側：UIから受け取ったプロジェクト名を取得  
SVC側：環境変数からプロジェクト作成場所取得  
SVC側：プロジェクト作成場所にプロジェクト名存在チェック  
SVC側：存在する場合 => 返却メッセージをセットして処理終了  
SVC側：存在しない場合 => 処理続行  
```rust

```

## クロスプラットフォームへの考慮
- WEBへの移行を容易にするためにコンソールとGUIでもプロジェクト情報管理にsqlite3を使う。  
GUIとconsoleの場合はプログラムからSQLiteデータベースを作成するためにrusqliteを使用してからDieselで操作する。  
webはPostgresを使用する。  
- 
- システムのコアな機能部分はcommonとしてAPIを開発して、それぞれのプラットフォーム開発ではそれらを呼ぶだけ、UIからのデータを渡すだけにする。
- WEBではGUI・コンソールと違い直接ディレクトリ構築や


## API
基本的に初期値で受け取るDB接続poolと設定情報をもとにファイル操作とDB操作を行う。

エラー用列挙子
```rust
enum PMError {
  DBError,
  FileError,
  ProjectError,
}
```

セッティング構造体
```rust
struct PMSetting {
 //暗号化有無
 //
}
```


プロジェクトマネージャ構造体
```rust
struct ProjectManager {
  conn: DatabaseConnection,
  setting: PMSetting,
}
```

プロジェクト新規作成
```rust
fn create_project(project_name: &str) {}
```

コマンド:プロジェクト削除
```rust
fn delete_project(project_name: &str) {}
```

コマンド:プロジェクト一覧表示
```rust
fn list_project() {}
```

コマンド:プロジェクト情報表示
```rust
fn display_project_info(project_name: &str) {}
```

コマンド:プロジェクトを開く
```rust
fn open_project(project_name: &str) {}
```

コマンド:環境設定の編集
```rust
fn create_project(project_name: &str) {}
```



