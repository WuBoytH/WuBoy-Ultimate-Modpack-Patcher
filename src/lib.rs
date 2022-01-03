#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

use arcropolis_api::*;
use prc::hash40::{Hash40, to_hash40};
use prc::*;

#[arc_callback]
fn my_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    load_original_file(hash, &mut data);
    // with the param data ready,
    let mut reader = std::io::Cursor::new(&mut data);
    let mut root = prc::read_stream(&mut reader).unwrap();

    // enter the first and only node of the file ("db_root")
    let (db_root_hash, db_root) = &mut root.0[0];
    assert_eq!(*db_root_hash, to_hash40("db_root"));

    let db_root_list = db_root.try_into_mut::<ParamList>().unwrap();

    // iterate the list to find the param with Lucina's data
    // we could go to the exact index, but this is subject to change across game updates.
    db_root_list.0.iter_mut().for_each(|param| {
        let ui_chara_struct = param.try_into_ref::<ParamStruct>().unwrap();
        let (_, ui_chara_id) = &ui_chara_struct.0[0];
        let ui_chara_hash = ui_chara_id.try_into_ref::<Hash40>().unwrap();
        if *ui_chara_hash == to_hash40("ui_chara_donkey") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n02_index") {
                    *param.try_into_mut::<u8>().unwrap() = 2;
                }
                else if *hash == to_hash40("n04_index") {
                    *param.try_into_mut::<u8>().unwrap() = 4;
                }
                else if *hash == to_hash40("characall_label_c02") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling_c01_article");
                }
                else if *hash == to_hash40("characall_label_c04") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_samusd") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_dsamus");
                }
                // else if *hash == to_hash40("characall_label_article_c00") {
                //     *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling_c01");
                // }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_pikachu") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                else if *hash == to_hash40("characall_label_c07") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_dsamus_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_captain") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                else if *hash == to_hash40("characall_label_c07") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_iceclimber_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_luigi") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                // else if *hash == to_hash40("characall_label_c07") {
                //     *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito_c01");
                // }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_purin") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n06_index") {
                    *param.try_into_mut::<u8>().unwrap() = 6;
                }
                else if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 6;
                }
                // else if *hash == to_hash40("characall_label_c06") {
                //     *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer_c01_article");
                // }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_sheik") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                else if *hash == to_hash40("characall_label_c07") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit_c01");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_ice_climber") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_iceclimber");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_koopa") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_koopa");
                }
                else if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 6;
                }
                else if *hash == to_hash40("characall_label_c06") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_duckhunt_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_pichu") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n02_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                else if *hash == to_hash40("characall_label_c07") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pickel_c07_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_lucina") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_persona");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_ganon") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 6;
                }
                else if *hash == to_hash40("characall_label_c06") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_younglink") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_younglink");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_wario") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                // else if *hash == to_hash40("characall_label_c07") {
                //     *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_krool_article");
                // }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_dedede") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_dedede");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_pitb") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pitb");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_snake") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n00_index") {
                    *param.try_into_mut::<u8>().unwrap() = 2;
                }
                else if *hash == to_hash40("characall_label_c02") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_c01_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_ike") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n01_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("n03_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("n05_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_ptrainer") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_pikmin") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n03_index") {
                    *param.try_into_mut::<u8>().unwrap() = 3;
                }
                // else if *hash == to_hash40("characall_label_c03") {
                //     *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit_article");
                // }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_koopajr") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_koopa");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_shulk") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n05_index") {
                    *param.try_into_mut::<u8>().unwrap() = 5;
                }
                else if *hash == to_hash40("characall_label_c05") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_murabito") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_murabito");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_wiifit") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_wiifit");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_reflet") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_reflet");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_palutena") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                else if *hash == to_hash40("characall_label_c07") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_duckhunt") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_duckhunt");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_pacman") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                else if *hash == to_hash40("characall_label_c07") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_packun_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_ryu") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 2;
                }
                else if *hash == to_hash40("characall_label_c02") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pitb_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_bayonetta") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n01_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("n03_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("n05_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 1;
                }
                else if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_dedede_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_inkling") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n04_index") {
                    *param.try_into_mut::<u8>().unwrap() = 4;
                }
                else if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 4;
                }
                else if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling");
                }
                else if *hash == to_hash40("characall_label_c04") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling_article");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling");
                }
                else if *hash == to_hash40("characall_label_article_c04") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_inkling_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_krool") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_krool");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_miifighter") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter_c01");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter_c01");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miifighter_c01");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_miiswordsman") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_c01");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_c01");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_c01");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_miigunner") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner_c01");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner_c01");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miigunner_c01");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_packun") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_packun");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_brave") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_c02") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer_article");
                }
                else if *hash == to_hash40("characall_label_article_c00") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave");
                }
                else if *hash == to_hash40("characall_label_article_c01") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave");
                }
                else if *hash == to_hash40("characall_label_article_c02") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer_article");
                }
                else if *hash == to_hash40("characall_label_article_c03") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_brave");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_dolly") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n04_index") {
                    *param.try_into_mut::<u8>().unwrap() = 4;
                }
                else if *hash == to_hash40("n05_index") {
                    *param.try_into_mut::<u8>().unwrap() = 5;
                }
                else if *hash == to_hash40("characall_label_c04") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_reflet_c01");
                }
                else if *hash == to_hash40("characall_label_c05") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_poketrainer_c01");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_master") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_fireemblemthreehouses");
                }
                else if *hash == to_hash40("n07_index") {
                    *param.try_into_mut::<u8>().unwrap() = 7;
                }
                // else if *hash == to_hash40("characall_label_c07") {
                //     *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword");
                // }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_pickel") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("characall_label_article_c06") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pickel_c06");
                }
                else if *hash == to_hash40("characall_label_article_c07") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pickel_c07");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_edge") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("n03_index") {
                    *param.try_into_mut::<u8>().unwrap() = 3;
                }
                else if *hash == to_hash40("n04_index") {
                    *param.try_into_mut::<u8>().unwrap() = 4;
                }
                else if *hash == to_hash40("characall_label_c03") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_miisword_article");
                }
                else if *hash == to_hash40("characall_label_c04") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("vc_narration_characall_pickel_c06_article");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_element") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_xenoblade2");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_flame_first") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_xenoblade2");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_light_first") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_xenoblade2");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_flame_only") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_xenoblade2");
                }
            });
        }
        else if *ui_chara_hash == to_hash40("ui_chara_light_only") {
            let char_struct = param.try_into_mut::<ParamStruct>().unwrap();
            char_struct.0.iter_mut().for_each(|(hash, param)| {
                if *hash == to_hash40("ui_series_id") {
                    *param.try_into_mut::<Hash40>().unwrap() = to_hash40("ui_series_xenoblade2");
                }
            });
        }
    });
    let mut writer = std::io::Cursor::new(data);
    write_stream(&mut writer, &root).unwrap();
    return Some(writer.position() as usize);
}

#[skyline::main(name = "wuboy_ultimate_modpack_patcher")]
pub fn main() {
    my_callback::install("ui/param/database/ui_chara_db.prc",  0xFFFF);
}