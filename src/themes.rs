#![allow(unused)]

use std::{collections::HashMap, sync::LazyLock};

static ARGONAUT: &[u8; include_bytes!("../themes/[ Argonaut ].tmTheme").len()] =
    include_bytes!("../themes/[ Argonaut ].tmTheme");
// static ACTIVE4D: &[u8; include_bytes!("../themes/Active4D.tmTheme").len()] =
//     include_bytes!("../themes/Active4D.tmTheme");
// static ALL_HALLOWS_EVE_CUSTOM: &[u8; include_bytes!("../themes/All Hallow's Eve Custom.tmTheme")
//      .len()] = include_bytes!("../themes/All Hallow's Eve Custom.tmTheme");
// static ALL_HALLOWS_EVE: &[u8; include_bytes!("../themes/All Hallow's Eve.tmTheme").len()] =
//     include_bytes!("../themes/All Hallow's Eve.tmTheme");
// static AMY: &[u8; include_bytes!("../themes/Amy.tmTheme").len()] =
//     include_bytes!("../themes/Amy.tmTheme");
// static BARF: &[u8; include_bytes!("../themes/barf.tmTheme").len()] =
//     include_bytes!("../themes/barf.tmTheme");
// static BBEDIT: &[u8; include_bytes!("../themes/BBEdit.tmTheme").len()] =
//     include_bytes!("../themes/BBEdit.tmTheme");
static BESPIN: &[u8; include_bytes!("../themes/Bespin.tmTheme").len()] =
    include_bytes!("../themes/Bespin.tmTheme");
// static BIRDS_OF_PARADISE: &[u8; include_bytes!("../themes/Birds of Paradise.tmTheme").len()] =
//     include_bytes!("../themes/Birds of Paradise.tmTheme");
// static BLACK_PEARL_II: &[u8; include_bytes!("../themes/Black Pearl II.tmTheme").len()] =
//     include_bytes!("../themes/Black Pearl II.tmTheme");
// static BLACK_PEARL: &[u8; include_bytes!("../themes/Black Pearl.tmTheme").len()] =
//     include_bytes!("../themes/Black Pearl.tmTheme");
// static BLACKBOARD_BLACK: &[u8; include_bytes!("../themes/Blackboard Black.tmTheme").len()] =
//     include_bytes!("../themes/Blackboard Black.tmTheme");
static BLACKBOARD_MOD: &[u8; include_bytes!("../themes/Blackboard Mod.tmTheme").len()] =
    include_bytes!("../themes/Blackboard Mod.tmTheme");
// static BLACKBOARD_NEW: &[u8; include_bytes!("../themes/Blackboard New.tmTheme").len()] =
//     include_bytes!("../themes/Blackboard New.tmTheme");
// static BLACKBOARD: &[u8; include_bytes!("../themes/Blackboard.tmTheme").len()] =
//     include_bytes!("../themes/Blackboard.tmTheme");
static BLACKLIGHT: &[u8; include_bytes!("../themes/BlackLight.tmTheme").len()] =
    include_bytes!("../themes/BlackLight.tmTheme");
// static BONGZILLA: &[u8; include_bytes!("../themes/Bongzilla.tmTheme").len()] =
//     include_bytes!("../themes/Bongzilla.tmTheme");
// static BOYS_GIRLS_0: &[u8; include_bytes!("../themes/Boys & Girls 0.1.tmTheme").len()] =
//     include_bytes!("../themes/Boys & Girls 0.1.tmTheme");
// static BRILLIANCE_BLACK: &[u8; include_bytes!("../themes/Brilliance Black.tmTheme").len()] =
//     include_bytes!("../themes/Brilliance Black.tmTheme");
// static BRILLIANCE_DULL: &[u8; include_bytes!("../themes/Brilliance Dull.tmTheme").len()] =
//     include_bytes!("../themes/Brilliance Dull.tmTheme");
// static CHOCO: &[u8; include_bytes!("../themes/choco.tmTheme").len()] =
//     include_bytes!("../themes/choco.tmTheme");
// static CLAIRE: &[u8; include_bytes!("../themes/Claire.tmTheme").len()] =
//     include_bytes!("../themes/Claire.tmTheme");
// static CLASSIC_MODIFIED: &[u8; include_bytes!("../themes/Classic Modified.tmTheme").len()] =
//     include_bytes!("../themes/Classic Modified.tmTheme");
// static CLOSE_TO_THE_SEA: &[u8; include_bytes!("../themes/close_to_the_sea.tmTheme").len()] =
//     include_bytes!("../themes/close_to_the_sea.tmTheme");
// static CLOUDS_MIDNIGHT: &[u8; include_bytes!("../themes/Clouds Midnight.tmTheme").len()] =
//     include_bytes!("../themes/Clouds Midnight.tmTheme");
// static CLOUDS: &[u8; include_bytes!("../themes/Clouds.tmTheme").len()] =
//     include_bytes!("../themes/Clouds.tmTheme");
// static COAL_GRAAL: &[u8; include_bytes!("../themes/Coal Graal.tmTheme").len()] =
//     include_bytes!("../themes/Coal Graal.tmTheme");
static COBALT: &[u8; include_bytes!("../themes/Cobalt.tmTheme").len()] =
    include_bytes!("../themes/Cobalt.tmTheme");
// static COOL_GLOW: &[u8; include_bytes!("../themes/Cool Glow.tmTheme").len()] =
//     include_bytes!("../themes/Cool Glow.tmTheme");
// static CREEPER: &[u8; include_bytes!("../themes/Creeper.tmTheme").len()] =
//     include_bytes!("../themes/Creeper.tmTheme");
// static CSSEDIT: &[u8; include_bytes!("../themes/CSSEdit.tmTheme").len()] =
//     include_bytes!("../themes/CSSEdit.tmTheme");
// static DANIEL_FISCHER: &[u8; include_bytes!("../themes/Daniel Fischer.tmTheme").len()] =
//     include_bytes!("../themes/Daniel Fischer.tmTheme");
// static DAWN_MOD1: &[u8; include_bytes!("../themes/Dawn mod1.tmTheme").len()] =
//     include_bytes!("../themes/Dawn mod1.tmTheme");
// static DAWN: &[u8; include_bytes!("../themes/Dawn.tmTheme").len()] =
//     include_bytes!("../themes/Dawn.tmTheme");
// static DELUXE: &[u8; include_bytes!("../themes/Deluxe.tmTheme").len()] =
//     include_bytes!("../themes/Deluxe.tmTheme");
// static DJANGO_SMOOTHY: &[u8; include_bytes!("../themes/Django (Smoothy).tmTheme").len()] =
//     include_bytes!("../themes/Django (Smoothy).tmTheme");
// static DJANGO_DARK: &[u8; include_bytes!("../themes/Django Dark.tmTheme").len()] =
//     include_bytes!("../themes/Django Dark.tmTheme");
// static DOMINION_DAY: &[u8; include_bytes!("../themes/Dominion Day.tmTheme").len()] =
//     include_bytes!("../themes/Dominion Day.tmTheme");
// static EIFFEL: &[u8; include_bytes!("../themes/Eiffel.tmTheme").len()] =
//     include_bytes!("../themes/Eiffel.tmTheme");
// static EMACS_STRICT: &[u8; include_bytes!("../themes/Emacs Strict.tmTheme").len()] =
//     include_bytes!("../themes/Emacs Strict.tmTheme");
// static EREBUS: &[u8; include_bytes!("../themes/Erebus.tmTheme").len()] =
//     include_bytes!("../themes/Erebus.tmTheme");
// static ESPRESSO_LIBRE: &[u8; include_bytes!("../themes/Espresso Libre.tmTheme").len()] =
//     include_bytes!("../themes/Espresso Libre.tmTheme");
// static ESPRESSO_TUTTI: &[u8; include_bytes!("../themes/Espresso Tutti.tmTheme").len()] =
//     include_bytes!("../themes/Espresso Tutti.tmTheme");
// static ESPRESSO: &[u8; include_bytes!("../themes/Espresso.tmTheme").len()] =
//     include_bytes!("../themes/Espresso.tmTheme");
// static FADE_TO_GREY: &[u8; include_bytes!("../themes/Fade to Grey.tmTheme").len()] =
//     include_bytes!("../themes/Fade to Grey.tmTheme");
static FAKE: &[u8; include_bytes!("../themes/fake.tmTheme").len()] =
    include_bytes!("../themes/fake.tmTheme");
// static FLUIDVISION: &[u8; include_bytes!("../themes/Fluidvision.tmTheme").len()] =
//     include_bytes!("../themes/Fluidvision.tmTheme");
// static FORLATEX: &[u8; include_bytes!("../themes/ForLaTeX.tmTheme").len()] =
//     include_bytes!("../themes/ForLaTeX.tmTheme");
// static FRECKLE_MOD1: &[u8; include_bytes!("../themes/Freckle mod1.tmTheme").len()] =
//     include_bytes!("../themes/Freckle mod1.tmTheme");
// static FRECKLE_MOD2: &[u8; include_bytes!("../themes/Freckle mod2.tmTheme").len()] =
//     include_bytes!("../themes/Freckle mod2.tmTheme");
// static FRECKLE: &[u8; include_bytes!("../themes/Freckle.tmTheme").len()] =
//     include_bytes!("../themes/Freckle.tmTheme");
// static FRIENDSHIP_BRACELET: &[u8; include_bytes!("../themes/Friendship Bracelet.tmTheme").len()] =
//     include_bytes!("../themes/Friendship Bracelet.tmTheme");
// static FUNKY_DASHBOARD: &[u8; include_bytes!("../themes/Funky_Dashboard.tmTheme").len()] =
//     include_bytes!("../themes/Funky_Dashboard.tmTheme");
// static GITHUB: &[u8; include_bytes!("../themes/GitHub.tmTheme").len()] =
//     include_bytes!("../themes/GitHub.tmTheme");
static GLITTERBOMB: &[u8; include_bytes!("../themes/GlitterBomb.tmTheme").len()] =
    include_bytes!("../themes/GlitterBomb.tmTheme");
// static GLOW: &[u8; include_bytes!("../themes/Glow.tmTheme").len()] =
//     include_bytes!("../themes/Glow.tmTheme");
// static HAPPY_HAPPY_JOY_JOY_2: &[u8; include_bytes!("../themes/Happy happy joy joy 2.tmTheme")
//      .len()] = include_bytes!("../themes/Happy happy joy joy 2.tmTheme");
// static HAPPYDELUXE: &[u8; include_bytes!("../themes/happydeluxe.tmTheme").len()] =
//     include_bytes!("../themes/happydeluxe.tmTheme");
// static HEROKU: &[u8; include_bytes!("../themes/Heroku.tmTheme").len()] =
//     include_bytes!("../themes/Heroku.tmTheme");
// static HEROKUCODESAMPLES: &[u8; include_bytes!("../themes/HerokuCodeSamples.tmTheme").len()] =
//     include_bytes!("../themes/HerokuCodeSamples.tmTheme");
// static IDLE: &[u8; include_bytes!("../themes/IDLE.tmTheme").len()] =
//     include_bytes!("../themes/IDLE.tmTheme");
// static IDLEFINGERS: &[u8; include_bytes!("../themes/idleFingers.tmTheme").len()] =
//     include_bytes!("../themes/idleFingers.tmTheme");
// static ILIFE_05: &[u8; include_bytes!("../themes/iLife 05.tmTheme").len()] =
//     include_bytes!("../themes/iLife 05.tmTheme");
// static ILIFE_06: &[u8; include_bytes!("../themes/iLife 06.tmTheme").len()] =
//     include_bytes!("../themes/iLife 06.tmTheme");
// static IMATHIS: &[u8; include_bytes!("../themes/imathis.tmTheme").len()] =
//     include_bytes!("../themes/imathis.tmTheme");
// static INKDEEP: &[u8; include_bytes!("../themes/inkdeep.tmTheme").len()] =
//     include_bytes!("../themes/inkdeep.tmTheme");
// static IPLASTIC: &[u8; include_bytes!("../themes/iPlastic.tmTheme").len()] =
//     include_bytes!("../themes/iPlastic.tmTheme");
// static IR_BLACK: &[u8; include_bytes!("../themes/IR_Black.tmTheme").len()] =
//     include_bytes!("../themes/IR_Black.tmTheme");
// static IR_WHITE: &[u8; include_bytes!("../themes/IR_White.tmTheme").len()] =
//     include_bytes!("../themes/IR_White.tmTheme");
static JUICY: &[u8; include_bytes!("../themes/Juicy.tmTheme").len()] =
    include_bytes!("../themes/Juicy.tmTheme");
// static KRTHEME: &[u8; include_bytes!("../themes/krTheme.tmTheme").len()] =
//     include_bytes!("../themes/krTheme.tmTheme");
// static LOWLIGHT: &[u8; include_bytes!("../themes/Lowlight.tmTheme").len()] =
//     include_bytes!("../themes/Lowlight.tmTheme");
// static MAC_CLASSIC: &[u8; include_bytes!("../themes/Mac Classic.tmTheme").len()] =
//     include_bytes!("../themes/Mac Classic.tmTheme");
// static MADE_OF_CODE: &[u8; include_bytes!("../themes/Made of Code.tmTheme").len()] =
//     include_bytes!("../themes/Made of Code.tmTheme");
// static MAGICWB_AMIGA: &[u8; include_bytes!("../themes/MagicWB (Amiga).tmTheme").len()] =
//     include_bytes!("../themes/MagicWB (Amiga).tmTheme");
// static MENAGE_A_TROIS: &[u8; include_bytes!("../themes/Menage A Trois.tmTheme").len()] =
//     include_bytes!("../themes/Menage A Trois.tmTheme");
// static MERBIVORE_SOFT: &[u8; include_bytes!("../themes/Merbivore Soft.tmTheme").len()] =
//     include_bytes!("../themes/Merbivore Soft.tmTheme");
// static MERBIVORE: &[u8; include_bytes!("../themes/Merbivore.tmTheme").len()] =
//     include_bytes!("../themes/Merbivore.tmTheme");
static MIDNIGHT: &[u8; include_bytes!("../themes/Midnight.tmTheme").len()] =
    include_bytes!("../themes/Midnight.tmTheme");
// static MINIMAL_THEME: &[u8; include_bytes!("../themes/minimal Theme.tmTheme").len()] =
//     include_bytes!("../themes/minimal Theme.tmTheme");
// static MONOINDUSTRIAL: &[u8; include_bytes!("../themes/monoindustrial.tmTheme").len()] =
//     include_bytes!("../themes/monoindustrial.tmTheme");
pub static MONOKAI_DARK: &[u8; include_bytes!("../themes/Monokai Dark.tmTheme").len()] =
    include_bytes!("../themes/Monokai Dark.tmTheme");
// static MONOKAI_FOR_TEXTMATERS_CUSTOM_PHILTR: &[u8; include_bytes!(
//     "../themes/Monokai for Textmaters CUSTOM (philtr).tmTheme"
// )
//  .len()] = include_bytes!("../themes/Monokai for Textmaters CUSTOM (philtr).tmTheme");
// static MONOKAI_FOR_TEXTMATERS_CUSTOM: &[u8; include_bytes!(
//     "../themes/Monokai for Textmaters CUSTOM.tmTheme"
// )
//  .len()] = include_bytes!("../themes/Monokai for Textmaters CUSTOM.tmTheme");
// static MONOKAI_MOD_SEANGAFFNEY: &[u8; include_bytes!(
//     "../themes/Monokai Mod (seangaffney).tmTheme"
// )
//  .len()] = include_bytes!("../themes/Monokai Mod (seangaffney).tmTheme");
// static MONOKAI_MOD_1: &[u8; include_bytes!("../themes/Monokai mod 1.tmTheme").len()] =
//     include_bytes!("../themes/Monokai mod 1.tmTheme");
// static MONOKAI_MOD: &[u8; include_bytes!("../themes/Monokai Mod.tmTheme").len()] =
//     include_bytes!("../themes/Monokai Mod.tmTheme");
// static MONOKAI: &[u8; include_bytes!("../themes/monokai.tmTheme").len()] =
//     include_bytes!("../themes/monokai.tmTheme");
// static MULTIMARKDOWN: &[u8; include_bytes!("../themes/MultiMarkdown.tmTheme").len()] =
//     include_bytes!("../themes/MultiMarkdown.tmTheme");
// static NOTEBOOK: &[u8; include_bytes!("../themes/Notebook.tmTheme").len()] =
//     include_bytes!("../themes/Notebook.tmTheme");
// static NOTEPAD2: &[u8; include_bytes!("../themes/Notepad2.tmTheme").len()] =
//     include_bytes!("../themes/Notepad2.tmTheme");
// static OFFY: &[u8; include_bytes!("../themes/Offy.tmTheme").len()] =
//     include_bytes!("../themes/Offy.tmTheme");
// static PACKAGEMETADATA: &[u8; include_bytes!("../themes/package-metadata.json").len()] =
//     include_bytes!("../themes/package-metadata.json");
// static PASTELS_ON_DARK: &[u8; include_bytes!("../themes/Pastels on Dark.tmTheme").len()] =
//     include_bytes!("../themes/Pastels on Dark.tmTheme");
// static PASTIE: &[u8; include_bytes!("../themes/Pastie.tmTheme").len()] =
//     include_bytes!("../themes/Pastie.tmTheme");
// static PENGWYNN_MENLO: &[u8; include_bytes!("../themes/Pengwynn menlo.tmTheme").len()] =
//     include_bytes!("../themes/Pengwynn menlo.tmTheme");
// static PENGWYNN: &[u8; include_bytes!("../themes/Pengwynn.tmTheme").len()] =
//     include_bytes!("../themes/Pengwynn.tmTheme");
// pub static PLUM_DUMB: &[u8; include_bytes!("../themes/Plum Dumb.tmTheme").len()] =
//     include_bytes!("../themes/Plum Dumb.tmTheme");
// static PUTTY: &[u8; include_bytes!("../themes/Putty.tmTheme").len()] =
//     include_bytes!("../themes/Putty.tmTheme");
// static RAILS_ENVY: &[u8; include_bytes!("../themes/Rails Envy.tmTheme").len()] =
//     include_bytes!("../themes/Rails Envy.tmTheme");
// static RAILSCASTS_BOOST: &[u8; include_bytes!("../themes/Railscasts - boost.tmTheme").len()] =
//     include_bytes!("../themes/Railscasts - boost.tmTheme");
// static RAILSCASTS: &[u8; include_bytes!("../themes/Railscasts.tmTheme").len()] =
//     include_bytes!("../themes/Railscasts.tmTheme");
// static RDARK: &[u8; include_bytes!("../themes/RDark.tmTheme").len()] =
//     include_bytes!("../themes/RDark.tmTheme");
// static README: &[u8; include_bytes!("../themes/README.md").len()] =
//     include_bytes!("../themes/README.md");
// static RESESIF: &[u8; include_bytes!("../themes/Resesif.tmTheme").len()] =
//     include_bytes!("../themes/Resesif.tmTheme");
// static RUBY_BLUE: &[u8; include_bytes!("../themes/Ruby Blue.tmTheme").len()] =
//     include_bytes!("../themes/Ruby Blue.tmTheme");
// static RUBYROBOT: &[u8; include_bytes!("../themes/RubyRobot.tmTheme").len()] =
//     include_bytes!("../themes/RubyRobot.tmTheme");
// static RYANLIGHT: &[u8; include_bytes!("../themes/ryan-light.tmTheme").len()] =
//     include_bytes!("../themes/ryan-light.tmTheme");
// static SIDEWALKCHALK: &[u8; include_bytes!("../themes/SidewalkChalk.tmTheme").len()] =
//     include_bytes!("../themes/SidewalkChalk.tmTheme");
// static SIDEWALKCHALKGREENMOD: &[u8; include_bytes!("../themes/SidewalkChalkGreenMod.tmTheme")
//      .len()] = include_bytes!("../themes/SidewalkChalkGreenMod.tmTheme");
// static SLUSH_POPPIES: &[u8; include_bytes!("../themes/Slush & Poppies.tmTheme").len()] =
//     include_bytes!("../themes/Slush & Poppies.tmTheme");
// static SMOOTHY: &[u8; include_bytes!("../themes/Smoothy.tmTheme").len()] =
//     include_bytes!("../themes/Smoothy.tmTheme");
// static SOLARIZED_DARK: &[u8; include_bytes!("../themes/Solarized (dark).tmTheme").len()] =
//     include_bytes!("../themes/Solarized (dark).tmTheme");
// static SOLARIZED_LIGHT: &[u8; include_bytes!("../themes/Solarized (light).tmTheme").len()] =
//     include_bytes!("../themes/Solarized (light).tmTheme");
// static SPACECADET: &[u8; include_bytes!("../themes/SpaceCadet.tmTheme").len()] =
//     include_bytes!("../themes/SpaceCadet.tmTheme");
static SPECTACULAR: &[u8; include_bytes!("../themes/Spectacular.tmTheme").len()] =
    include_bytes!("../themes/Spectacular.tmTheme");
// static STARLIGHT: &[u8; include_bytes!("../themes/Starlight.tmTheme").len()] =
//     include_bytes!("../themes/Starlight.tmTheme");
// static SUCCULENT_1: &[u8; include_bytes!("../themes/Succulent_1.tmTheme").len()] =
//     include_bytes!("../themes/Succulent_1.tmTheme");
// static SUMMER_CAMP_DAYBREAK: &[u8; include_bytes!("../themes/Summer Camp Daybreak.tmTheme").len()] =
//     include_bytes!("../themes/Summer Camp Daybreak.tmTheme");
// static SUMMER_CAMP_MOD: &[u8; include_bytes!("../themes/Summer Camp Mod.tmTheme").len()] =
//     include_bytes!("../themes/Summer Camp Mod.tmTheme");
// static SUMMER_SUN: &[u8; include_bytes!("../themes/Summer Sun.tmTheme").len()] =
//     include_bytes!("../themes/Summer Sun.tmTheme");
// static SUNBURST: &[u8; include_bytes!("../themes/Sunburst.tmTheme").len()] =
//     include_bytes!("../themes/Sunburst.tmTheme");
// static SWEYLA650478: &[u8; include_bytes!("../themes/Sweyla650478.tmTheme").len()] =
//     include_bytes!("../themes/Sweyla650478.tmTheme");
// static SWEYLA674314: &[u8; include_bytes!("../themes/Sweyla674314.tmTheme").len()] =
//     include_bytes!("../themes/Sweyla674314.tmTheme");
// static SWYPHS_II: &[u8; include_bytes!("../themes/Swyphs II.tmTheme").len()] =
//     include_bytes!("../themes/Swyphs II.tmTheme");
// static TANGO_IN_TWILIGHT: &[u8; include_bytes!("../themes/Tango in Twilight.tmTheme").len()] =
//     include_bytes!("../themes/Tango in Twilight.tmTheme");
// static TANGO: &[u8; include_bytes!("../themes/Tango.tmTheme").len()] =
//     include_bytes!("../themes/Tango.tmTheme");
// static TEK: &[u8; include_bytes!("../themes/Tek.tmTheme").len()] =
//     include_bytes!("../themes/Tek.tmTheme");
// static TEXT_EX_MACHINA_LIGHTER_COMMENTS: &[u8; include_bytes!(
//     "../themes/Text Ex Machina (Lighter comments).tmTheme"
// )
//  .len()] = include_bytes!("../themes/Text Ex Machina (Lighter comments).tmTheme");
// static TEXT_EX_MACHINA: &[u8; include_bytes!("../themes/Text Ex Machina.tmTheme").len()] =
//     include_bytes!("../themes/Text Ex Machina.tmTheme");
// static THEMES: &[u8; include_bytes!("../themes/themes.txt").len()] =
//     include_bytes!("../themes/themes.txt");
// static TOMORROW_NIGHT: &[u8; include_bytes!("../themes/Tomorrow Night.tmTheme").len()] =
//     include_bytes!("../themes/Tomorrow Night.tmTheme");
// static TOMORROWNIGHTBLUE: &[u8; include_bytes!("../themes/Tomorrow-Night-Blue.tmTheme").len()] =
//     include_bytes!("../themes/Tomorrow-Night-Blue.tmTheme");
// static TOMORROWNIGHTBRIGHT: &[u8; include_bytes!("../themes/Tomorrow-Night-Bright.tmTheme").len()] =
//     include_bytes!("../themes/Tomorrow-Night-Bright.tmTheme");
// static TOMORROWNIGHTEIGHTIES: &[u8; include_bytes!("../themes/Tomorrow-Night-Eighties.tmTheme")
//      .len()] = include_bytes!("../themes/Tomorrow-Night-Eighties.tmTheme");
// static TOMORROWNIGHT: &[u8; include_bytes!("../themes/Tomorrow-Night.tmTheme").len()] =
//     include_bytes!("../themes/Tomorrow-Night.tmTheme");
// static TOMORROW: &[u8; include_bytes!("../themes/Tomorrow.tmTheme").len()] =
//     include_bytes!("../themes/Tomorrow.tmTheme");
// static TUBSTER: &[u8; include_bytes!("../themes/Tubster.tmTheme").len()] =
//     include_bytes!("../themes/Tubster.tmTheme");
// static TWILIGHT_BRIGHT: &[u8; include_bytes!("../themes/Twilight Bright.tmTheme").len()] =
//     include_bytes!("../themes/Twilight Bright.tmTheme");
// static TWILIGHT_REMIX: &[u8; include_bytes!("../themes/Twilight REMIX.tmTheme").len()] =
//     include_bytes!("../themes/Twilight REMIX.tmTheme");
// static TWILIGHT: &[u8; include_bytes!("../themes/Twilight.tmTheme").len()] =
//     include_bytes!("../themes/Twilight.tmTheme");
// static UPSTREAM_SUNBURST: &[u8; include_bytes!("../themes/Upstream Sunburst.tmTheme").len()] =
//     include_bytes!("../themes/Upstream Sunburst.tmTheme");
// static UPSTREAM_VIBRANT: &[u8; include_bytes!("../themes/Upstream Vibrant.tmTheme").len()] =
//     include_bytes!("../themes/Upstream Vibrant.tmTheme");
// static VENOM: &[u8; include_bytes!("../themes/Venom.tmTheme").len()] =
//     include_bytes!("../themes/Venom.tmTheme");
// static VIBRANT_FIN: &[u8; include_bytes!("../themes/Vibrant Fin.tmTheme").len()] =
//     include_bytes!("../themes/Vibrant Fin.tmTheme");
// static VIBRANT_INK_CHOPPEDNSCREWED: &[u8; include_bytes!(
//     "../themes/Vibrant Ink chopped'n'screwed.tmTheme"
// )
//  .len()] = include_bytes!("../themes/Vibrant Ink chopped'n'screwed.tmTheme");
// static VIBRANT_INK_REMIX: &[u8; include_bytes!("../themes/Vibrant Ink remix.tmTheme").len()] =
//     include_bytes!("../themes/Vibrant Ink remix.tmTheme");
// static VIBRANT_INK: &[u8; include_bytes!("../themes/Vibrant Ink.tmTheme").len()] =
//     include_bytes!("../themes/Vibrant Ink.tmTheme");
// static VIBRANT_TANGO: &[u8; include_bytes!("../themes/Vibrant Tango.tmTheme").len()] =
//     include_bytes!("../themes/Vibrant Tango.tmTheme");
// static VINTAGE_AURORA: &[u8; include_bytes!("../themes/Vintage Aurora.tmTheme").len()] =
//     include_bytes!("../themes/Vintage Aurora.tmTheme");
// static WHYS_POIGNANT: &[u8; include_bytes!("../themes/Whys Poignant.tmTheme").len()] =
//     include_bytes!("../themes/Whys Poignant.tmTheme");
// static ZACHSTRONAUT_THEME_4: &[u8; include_bytes!("../themes/Zachstronaut Theme 4.1.tmTheme")
//      .len()] = include_bytes!("../themes/Zachstronaut Theme 4.1.tmTheme");
// static ZENBURN: &[u8; include_bytes!("../themes/zenburn.tmTheme").len()] =
//     include_bytes!("../themes/zenburn.tmTheme");
// static ZENBURNESQUE: &[u8; include_bytes!("../themes/Zenburnesque.tmTheme").len()] =
//     include_bytes!("../themes/Zenburnesque.tmTheme");

pub static THEME_MAP: LazyLock<HashMap<&str, &[u8]>> =
    LazyLock::<HashMap<&str, &[u8]>>::new(|| {
        let mut theme_map: HashMap<&str, &[u8]> = HashMap::new();

        theme_map.insert("ARGONAUT", ARGONAUT);
        // theme_map.insert("ACTIVE4D", ACTIVE4D);
        // theme_map.insert("ALL_HALLOWS_EVE_CUSTOM", ALL_HALLOWS_EVE_CUSTOM);
        // theme_map.insert("ALL_HALLOWS_EVE", ALL_HALLOWS_EVE);
        // theme_map.insert("AMY", AMY);
        // theme_map.insert("BARF", BARF);
        // theme_map.insert("BBEDIT", BBEDIT);
        theme_map.insert("BESPIN", BESPIN);
        // theme_map.insert("BIRDS_OF_PARADISE", BIRDS_OF_PARADISE);
        // theme_map.insert("BLACK_PEARL_II", BLACK_PEARL_II);
        // theme_map.insert("BLACK_PEARL", BLACK_PEARL);
        // theme_map.insert("BLACKBOARD_BLACK", BLACKBOARD_BLACK);
        theme_map.insert("BLACKBOARD_MOD", BLACKBOARD_MOD);
        // theme_map.insert("BLACKBOARD_NEW", BLACKBOARD_NEW);
        // theme_map.insert("BLACKBOARD", BLACKBOARD);
        theme_map.insert("BLACKLIGHT", BLACKLIGHT);
        // theme_map.insert("BONGZILLA", BONGZILLA);
        // theme_map.insert("BOYS_GIRLS_0", BOYS_GIRLS_0);
        // theme_map.insert("BRILLIANCE_BLACK", BRILLIANCE_BLACK);
        // theme_map.insert("BRILLIANCE_DULL", BRILLIANCE_DULL);
        // theme_map.insert("CHOCO", CHOCO);
        // theme_map.insert("CLAIRE", CLAIRE);
        // theme_map.insert("CLASSIC_MODIFIED", CLASSIC_MODIFIED);
        // theme_map.insert("CLOSE_TO_THE_SEA", CLOSE_TO_THE_SEA);
        // theme_map.insert("CLOUDS_MIDNIGHT", CLOUDS_MIDNIGHT);
        // theme_map.insert("CLOUDS", CLOUDS);
        // theme_map.insert("COAL_GRAAL", COAL_GRAAL);
        theme_map.insert("COBALT", COBALT);
        // theme_map.insert("COOL_GLOW", COOL_GLOW);
        // theme_map.insert("CREEPER", CREEPER);
        // theme_map.insert("CSSEDIT", CSSEDIT);
        // theme_map.insert("DANIEL_FISCHER", DANIEL_FISCHER);
        // theme_map.insert("DAWN_MOD1", DAWN_MOD1);
        // theme_map.insert("DAWN", DAWN);
        // theme_map.insert("DELUXE", DELUXE);
        // theme_map.insert("DJANGO_SMOOTHY", DJANGO_SMOOTHY);
        // theme_map.insert("DJANGO_DARK", DJANGO_DARK);
        // theme_map.insert("DOMINION_DAY", DOMINION_DAY);
        // theme_map.insert("EIFFEL", EIFFEL);
        // theme_map.insert("EMACS_STRICT", EMACS_STRICT);
        // theme_map.insert("EREBUS", EREBUS);
        // theme_map.insert("ESPRESSO_LIBRE", ESPRESSO_LIBRE);
        // theme_map.insert("ESPRESSO_TUTTI", ESPRESSO_TUTTI);
        // theme_map.insert("ESPRESSO", ESPRESSO);
        // theme_map.insert("FADE_TO_GREY", FADE_TO_GREY);
        theme_map.insert("FAKE", FAKE);
        // theme_map.insert("FLUIDVISION", FLUIDVISION);
        // theme_map.insert("FORLATEX", FORLATEX);
        // theme_map.insert("FRECKLE_MOD1", FRECKLE_MOD1);
        // theme_map.insert("FRECKLE_MOD2", FRECKLE_MOD2);
        // theme_map.insert("FRECKLE", FRECKLE);
        // theme_map.insert("FRIENDSHIP_BRACELET", FRIENDSHIP_BRACELET);
        // theme_map.insert("FUNKY_DASHBOARD", FUNKY_DASHBOARD);
        // theme_map.insert("GITHUB", GITHUB);
        theme_map.insert("GLITTERBOMB", GLITTERBOMB);
        // theme_map.insert("GLOW", GLOW);
        // theme_map.insert("HAPPY_HAPPY_JOY_JOY_2", HAPPY_HAPPY_JOY_JOY_2);
        // theme_map.insert("HAPPYDELUXE", HAPPYDELUXE);
        // theme_map.insert("HEROKU", HEROKU);
        // theme_map.insert("HEROKUCODESAMPLES", HEROKUCODESAMPLES);
        // theme_map.insert("IDLE", IDLE);
        // theme_map.insert("IDLEFINGERS", IDLEFINGERS);
        // theme_map.insert("ILIFE_05", ILIFE_05);
        // theme_map.insert("ILIFE_06", ILIFE_06);
        // theme_map.insert("IMATHIS", IMATHIS);
        // theme_map.insert("INKDEEP", INKDEEP);
        // theme_map.insert("IPLASTIC", IPLASTIC);
        // theme_map.insert("IR_BLACK", IR_BLACK);
        // theme_map.insert("IR_WHITE", IR_WHITE);
        theme_map.insert("JUICY", JUICY);
        // theme_map.insert("KRTHEME", KRTHEME);
        // theme_map.insert("LOWLIGHT", LOWLIGHT);
        // theme_map.insert("MAC_CLASSIC", MAC_CLASSIC);
        // theme_map.insert("MADE_OF_CODE", MADE_OF_CODE);
        // theme_map.insert("MAGICWB_AMIGA", MAGICWB_AMIGA);
        // theme_map.insert("MENAGE_A_TROIS", MENAGE_A_TROIS);
        // theme_map.insert("MERBIVORE_SOFT", MERBIVORE_SOFT);
        // theme_map.insert("MERBIVORE", MERBIVORE);
        theme_map.insert("MIDNIGHT", MIDNIGHT);
        // theme_map.insert("MINIMAL_THEME", MINIMAL_THEME);
        // theme_map.insert("MONOINDUSTRIAL", MONOINDUSTRIAL);
        theme_map.insert("MONOKAI_DARK", MONOKAI_DARK);
        // theme_map.insert(
        //     "MONOKAI_FOR_TEXTMATERS_CUSTOM_PHILTR",
        //     MONOKAI_FOR_TEXTMATERS_CUSTOM_PHILTR,
        // );
        // theme_map.insert(
        //     "MONOKAI_FOR_TEXTMATERS_CUSTOM",
        //     MONOKAI_FOR_TEXTMATERS_CUSTOM,
        // );
        // theme_map.insert("MONOKAI_MOD_SEANGAFFNEY", MONOKAI_MOD_SEANGAFFNEY);
        // theme_map.insert("MONOKAI_MOD_1", MONOKAI_MOD_1);
        // theme_map.insert("MONOKAI_MOD", MONOKAI_MOD);
        // theme_map.insert("MONOKAI", MONOKAI);
        // theme_map.insert("MULTIMARKDOWN", MULTIMARKDOWN);
        // theme_map.insert("NOTEBOOK", NOTEBOOK);
        // theme_map.insert("NOTEPAD2", NOTEPAD2);
        // theme_map.insert("OFFY", OFFY);
        // theme_map.insert("PACKAGEMETADATA", PACKAGEMETADATA);
        // theme_map.insert("PASTELS_ON_DARK", PASTELS_ON_DARK);
        // theme_map.insert("PASTIE", PASTIE);
        // theme_map.insert("PENGWYNN_MENLO", PENGWYNN_MENLO);
        // theme_map.insert("PENGWYNN", PENGWYNN);
        // theme_map.insert("PLUM_DUMB", PLUM_DUMB);
        // theme_map.insert("PUTTY", PUTTY);
        // theme_map.insert("RAILS_ENVY", RAILS_ENVY);
        // theme_map.insert("RAILSCASTS_BOOST", RAILSCASTS_BOOST);
        // theme_map.insert("RAILSCASTS", RAILSCASTS);
        // theme_map.insert("RDARK", RDARK);
        // theme_map.insert("README", README);
        // theme_map.insert("RESESIF", RESESIF);
        // theme_map.insert("RUBY_BLUE", RUBY_BLUE);
        // theme_map.insert("RUBYROBOT", RUBYROBOT);
        // theme_map.insert("RYANLIGHT", RYANLIGHT);
        // theme_map.insert("SIDEWALKCHALK", SIDEWALKCHALK);
        // theme_map.insert("SIDEWALKCHALKGREENMOD", SIDEWALKCHALKGREENMOD);
        // theme_map.insert("SLUSH_POPPIES", SLUSH_POPPIES);
        // theme_map.insert("SMOOTHY", SMOOTHY);
        // theme_map.insert("SOLARIZED_DARK", SOLARIZED_DARK);
        // theme_map.insert("SOLARIZED_LIGHT", SOLARIZED_LIGHT);
        // theme_map.insert("SPACECADET", SPACECADET);

        theme_map.insert("SPECTACULAR", SPECTACULAR);

        theme_map
    });
