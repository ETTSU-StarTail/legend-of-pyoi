// 元ネタ: JV-Linkインターフェース仕様書_4.8.0(Win).pdf, サンプルプログラム

// FFI どうやるんじゃ？ -> Win32API によるアプローチに変更
// NOTE: jv-link cls id = 2ab1774d-0c41-11d7-916f-0003479beb3f

use windows::{core::GUID, w, Win32::System::Com::CLSIDFromProgID};

/// IJVLink を再現する構造体
struct IJVLinkProperty {
    /// m_saveflag<br/><br/>
    /// サーバからダウンロードしたファイルを m_save_path に保存するかどうかのフラグ<br/><br/>
    /// 0: 保存しない<br/>
    /// 1: 保存する
    m_save_flag: i32,

    /// m_savepath<br/><br/>
    /// JV-Data を保存するディレクトリへのパス<br/><br/>
    /// jv_init 呼出時にレジストリから値をセットする(デフォルト: %InstallPath%)<br/>
    /// JV-Data はこのパスの配下に作成されるディレクトリ cache と data に保存される<br/>
    /// 値を変更する場合は jv_set_save_path() または jv_set_ui_properties() を使用する
    m_save_path: String,

    /// m_servicekey<br/><br/>
    /// JRA-VAN DataLab. サービスを利用する権利を確認する為の利用キー(17 桁)<br/><br/>
    /// jv_init() 呼出時にレジストリから値がセットされる(デフォルト: 未設定)<br/>
    /// 値を変更する場合は jv_set_service_key() または jv_set_ui_properties を使用する
    m_service_key: String,

    /// m_JVLinkVersion<br/><br/>
    /// JV-Link のバージョン(4 桁数字, ex: 0100)<br/><br/>
    /// 値は変更不可
    m_jvlink_version: String,

    /// m_TotalReadFilesize<br/><br/>
    /// jv_open() 呼出から戻った際にこれから読み込む JV-Data のそうデータサイズを 1024 で割った値<br/><br/>
    /// 結果が 0 の場合は 1 がセットされる<br/>
    /// jv_read() もしくは jv_gets() から 0 が返るまで読み取るデータの合計サイズとなる
    m_total_read_file_size: i32,

    /// m_CurrentReadFilesize<br/><br/>
    /// jv_read() もしくは jv_gets() で読み込んでいる現在のファイルのサイズ<br/><br/>
    /// 値は変更不可<br/>
    /// jv_open() 誤の最初の jv_read() もしくは jv_gets() 呼出でセットされ、 <br/>
    /// jv_read() もしくは jv_gets() から -1 が返るまで同じ値を維持する<br/>
    /// -1 が返った次の jv_read() もしくは jv_gets() の呼出で次のファイルのサイズに変更される
    m_current_read_file_size: i32,

    /// m_CurrentFileTimestamp<br/><br/>
    /// jv_read() もしくは jv_gets() で読み込んでいる現在のファイルのタイムスタンプ<br/><br/>
    /// jv_open() 呼出後の最初の jv_read() もしくは jv_gets() の呼出でセットされ、<br/>
    /// jv_read() もしくは jv_gets() から -1 が返るまで同じ値を維持する<br/>
    /// -1 が返った次の jv_read() もしくは jv_gets() の呼出で次のファイルのタイムスタンプに変更される
    m_current_file_timestamp: String,

    /// ParentHWnd<br/><br/>
    /// JV-Link が表示するメッセージダイアログのオーナーウィンドウ<br/><br/>
    /// jv_open() もしくは jv_real_time_open() の呼出前に設定すること<br/>
    /// ※JV-Link Version >=2.0.0 である必要有
    parent_window: String,

    /// m_payflag<br/><br/>
    /// 払戻ダイアログを表示するかどうかのフラグ<br/><br/>
    /// 0: 表示する<br/>
    /// 1: 表示しない
    m_pay_flag: i32,
}

/// IJVLink を再現するトレイト
trait IJVLinkMethod {
    ///
    fn jv_init() -> i32;
    fn jv_set_ui_properties() -> i32;
    fn jv_set_service_key() -> i32;
    fn jv_set_save_flag() -> i32;
    fn jv_set_save_path() -> i32;
    fn jv_set_pay_flag() -> i32;
    fn jv_open() -> i32;
    fn jv_real_time_open() -> i32;
    fn jv_status() -> i32;
    fn jv_read() -> i32;
    fn jv_gets() -> i32;
    fn jv_skip() -> i32;
    fn jv_cancel() -> i32;
    fn jv_close() -> i32;
    fn jv_file_delete() -> i32;
    /// JVFukuFile:
    fn jv_cloth_file() -> i32;
    fn jv_cloth() -> i32;
    fn jv_movie_check() -> i32;
    fn jv_movie_check_with_type() -> i32;
    fn jv_movie_play() -> i32;
    fn jv_movie_play_with_type() -> i32;
    fn jv_movie_open() -> i32;
    fn jv_movie_read() -> i32;
    fn jv_course_file() -> i32;
    fn jv_course_file_2() -> i32;
    fn jv_watch_event() -> i32;
    fn jv_watch_event_close() -> i32;
}

/// recreate AxJVLink class
#[derive(Debug)]
pub struct AxJVLink {
    pub class_id: GUID,
}

impl AxJVLink {
    pub fn new() -> AxJVLink {
        return AxJVLink {
            class_id: get_class_id().unwrap(),
        };
    }
}

fn get_class_id() -> Result<GUID, windows::core::Error> {
    unsafe {
        let class_id: GUID = CLSIDFromProgID(w!("JVDTLab.JVLink"))?;
        Ok(class_id)
    }
}
