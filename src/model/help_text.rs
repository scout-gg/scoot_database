use crate::game_data::key_value::Ao2KeyValues;
use crate::schema::help_text;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use eyre::Result;

#[derive(Queryable, Insertable, Associations, PartialEq, Debug)]
#[table_name = "help_text"]

pub struct HelpText {
    pub id: i32,
    pub content_en: String,
    pub content_fr: Option<String>,
    pub content_br: Option<String>,
    pub content_de: Option<String>,
    pub content_es: Option<String>,
    pub content_hi: Option<String>,
    pub content_it: Option<String>,
    pub content_jp: Option<String>,
    pub content_ko: Option<String>,
    pub content_ms: Option<String>,
    pub content_mx: Option<String>,
    pub content_ru: Option<String>,
    pub content_tr: Option<String>,
    pub content_tw: Option<String>,
    pub content_vi: Option<String>,
    pub content_zh: Option<String>,
}

impl HelpText {
    pub fn insert_from_values(
        conn: &PgConnection,
        values: &Ao2KeyValues,
        idx: i32,
    ) -> Result<Self> {
        if values.en.get(&idx).is_none() {
            Err(eyre!("Could not insert help text {}, no value found", idx))
        } else {
            let text = HelpText {
                id: idx,
                content_en: values.en.get(&idx).cloned().unwrap(),
                content_fr: values.fr.get(&idx).cloned(),
                content_br: values.br.get(&idx).cloned(),
                content_de: values.de.get(&idx).cloned(),
                content_es: values.es.get(&idx).cloned(),
                content_hi: values.hi.get(&idx).cloned(),
                content_it: values.it.get(&idx).cloned(),
                content_jp: values.jp.get(&idx).cloned(),
                content_ko: values.ko.get(&idx).cloned(),
                content_ms: values.ms.get(&idx).cloned(),
                content_mx: values.mx.get(&idx).cloned(),
                content_ru: values.ru.get(&idx).cloned(),
                content_tr: values.tr.get(&idx).cloned(),
                content_tw: values.tw.get(&idx).cloned(),
                content_vi: values.vi.get(&idx).cloned(),
                content_zh: values.zh.get(&idx).cloned(),
            };

            HelpText::insert(conn, &text)
        }
    }

    pub fn insert(conn: &PgConnection, text: &HelpText) -> Result<HelpText> {
        diesel::insert_into(help_text::table)
            .values(text)
            .get_result(conn)
            .map_err(|err| {
                eyre!(
                    "Error inserting help_text {} with id {} : {}",
                    text.content_en,
                    text.id,
                    err
                )
            })
            .or_else(|_| HelpText::by_id(conn, text.id))
    }

    pub fn by_id(conn: &PgConnection, id: i32) -> Result<HelpText> {
        help_text::table
            .find(id)
            .first(conn)
            .map_err(|err| eyre!("Building with id {} not found : {}", id, err))
    }
}
