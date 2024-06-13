use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ErrorReview {
    CantidadDeCamposInvalida,
    FechaEnFormatoInvalido,
    SinNombreDeUsuario,
    SinComentario,
    ScoreNoUnNumero,
    ThumbsUpNoUnNumero,
    IdFormatoInvalido,
    VersionAppFormatoInvalido,
}

#[derive(Debug)]
pub struct Review {
    pub id: String,
    pub user_name: String,
    pub content: String,
    pub score: i32,
    pub thumbs_up: i32,
    pub date: NaiveDateTime,
    pub app_version: String,
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"
    )
    .expect("Deberia ser un regex valido");
    static ref RE_APP_VERSION: Regex =
        Regex::new(r"^\d+\.\d+\.\d+ build \d+ \d+$").expect("Deberia ser un regex valido");
}
impl Review {
    pub fn new(linea: &str, sep: &str) -> Result<Self, ErrorReview> {
        let fields: Vec<&str> = linea.split(sep).collect();
        if fields.len() <= 6 {
            return Err(ErrorReview::CantidadDeCamposInvalida);
        }

        Ok(Self {
            id: Self::get_id(fields[0])?,
            user_name: Self::get_user_name(fields[1])?,
            content: Self::get_comment(fields[2])?,
            score: Self::get_score(fields[3])?,
            thumbs_up: Self::get_thumbs_up(fields[4])?,
            date: Self::get_date(&fields[6])?,
            app_version: Self::get_app_version(fields[5])?,
        })
    }

    fn get_id(id: &str) -> Result<String, ErrorReview> {
        if RE.is_match(id) {
            Ok(id.to_string())
        } else {
            Err(ErrorReview::IdFormatoInvalido)
        }
    }

    fn get_user_name(user_name: &str) -> Result<String, ErrorReview> {
        if user_name == "" {
            Err(ErrorReview::SinNombreDeUsuario)
        } else {
            Ok(user_name.to_string())
        }
    }

    fn get_comment(comment: &str) -> Result<String, ErrorReview> {
        if comment == "" {
            Err(ErrorReview::SinComentario)
        } else {
            Ok(comment.to_string())
        }
    }

    //Agregar validaciones extra? Por ejemplo si esperamos tener reviews de cierto año en adelante y demás
    fn get_date(date: &str) -> Result<NaiveDateTime, ErrorReview> {
        match NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S") {
            Ok(date) => Ok(date),
            Err(_) => Err(ErrorReview::FechaEnFormatoInvalido),
        }
    }

    //Cuando se lee, si no hay nada, por default se pone -1, así que habría que descartar estas líneas
    fn get_score(score: &str) -> Result<i32, ErrorReview> {
        match score.parse() {
            Ok(score) => Ok(score),
            Err(_) => Err(ErrorReview::ScoreNoUnNumero),
        }
    }

    //Cuando se lee, si no hay nada, por default se pone -1, así que habría que descartar estas líneas
    fn get_thumbs_up(thumbs_up: &str) -> Result<i32, ErrorReview> {
        match thumbs_up.parse() {
            Ok(thumbs_up) => Ok(thumbs_up),
            Err(_) => Err(ErrorReview::ThumbsUpNoUnNumero),
        }
    }

    fn get_app_version(app_version: &str) -> Result<String, ErrorReview> {
        if RE_APP_VERSION.is_match(app_version) {
            Ok(app_version.to_string())
        } else {
            Err(ErrorReview::VersionAppFormatoInvalido)
        }
    }
}

impl Display for Review {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "id: {}\nuser_name: {}\ncontent: {}\nscore: {}\nthumbs_up: {}\ndate: {}\napp_version: {}", 
            self.id,
            self.user_name,
            self.content,
            self.score,
            self.thumbs_up,
            self.date,
            self.app_version
        )
    }
}
