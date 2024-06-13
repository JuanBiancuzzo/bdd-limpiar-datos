use chrono::NaiveDateTime;


#[derive(Debug)]
pub enum RecordError {
    CantidadDeCamposInvalida,
    FechaEnFormatoInvalido,
    ScoreNoUnNumero,
    ThumbsUpNoUnNumero,
}
//reviewId,userName,content,score,thumbsUpCount,date,appVersion
#[derive(Debug)]
pub struct Record {
    pub id: String,
    pub user_name: String,
    pub content: String,
    pub score: i32,
    pub thumbs_up: i32,
    pub date: NaiveDateTime,
    pub app_version: String
}

impl Record {
    pub fn new(linea: &str, sep: &str) -> Result<Self, RecordError> {
        let fields: Vec<&str> = linea.split(sep).collect();
        
        if fields.len() < 7 {
            return Err(RecordError::CantidadDeCamposInvalida)
        }

        Ok(Self {
            id: fields[0].to_string(),
            user_name: fields[1].to_string(),
            content: fields[2].to_string(),
            score: fields[3].parse::<i32>().map_err(|_| RecordError::ScoreNoUnNumero)?,
            thumbs_up: fields[4].parse::<i32>().map_err(|_| RecordError::ThumbsUpNoUnNumero)?,
            date: NaiveDateTime::parse_from_str(fields[5], "%Y-%m-%d %H:%M:%S").map_err(|_| RecordError::FechaEnFormatoInvalido)?,
            app_version: fields[6].to_string()
        })
    }
}