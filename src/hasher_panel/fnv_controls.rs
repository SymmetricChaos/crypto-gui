use super::HasherFrame;
use crate::ui_elements::UiElements;
use hashers::{
    fnv::{Fnv1024, Fnv128, Fnv256, Fnv32, Fnv512, Fnv64, FnvVariant},
    traits::StatefulHasher,
};
use strum::IntoEnumIterator;
use utils::byte_formatting::ByteFormat;

pub struct FnvFrame {
    input_format: ByteFormat,
    output_format: ByteFormat,
    variant: FnvVariant,
    alternate: bool,
    zero_basis: bool,
}

impl Default for FnvFrame {
    fn default() -> Self {
        Self {
            input_format: ByteFormat::Utf8,
            output_format: ByteFormat::Hex,
            variant: FnvVariant::L256,
            alternate: true,
            zero_basis: false,
        }
    }
}

impl HasherFrame for FnvFrame {
    fn ui(&mut self, ui: &mut egui::Ui, _errors: &mut String) {
        ui.hyperlink_to(
            "see the code",
            "https://github.com/SymmetricChaos/crypto-gui/blob/master/hashers/src/fnv.rs",
        );
        ui.add_space(8.0);

        ui.byte_io_mode_hasher(&mut self.input_format, &mut self.output_format);

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            for variant in FnvVariant::iter() {
                ui.selectable_value(&mut self.variant, variant, variant.to_string());
            }
        });

        ui.add_space(16.0);
        ui.label("In the original FNV-1 algorithm the multiplication was performed before the XOR but better results were found when using the XOR first. The algorithms with this alternate order are known as FNV-1a.");
        ui.checkbox(&mut self.alternate, "Use Alternate Mode (recommended)");

        ui.add_space(8.0);
        ui.label("Zero basis mode initializes the hash with all zeroes and when used this way the hasher is called FNV-0. The usual initialization value for FNV-1 and (FNV-1a) was created by hashing the ASCII string \"chongo <Landon Curt Noll> /\\../\\\" in zero basis mode.");
        ui.checkbox(
            &mut self.zero_basis,
            "Use Zero Basis Mode (not recommended)",
        );

        ui.add_space(16.0);
        ui.subheading("Hash Size");
        ui.label("The FNV primes are of a specific form, close to a power of 256, which the developers found to be highly effective at dispersing the bits of the input throughout the state.");
        ui.add_space(4.0);
        match self.variant {
            FnvVariant::L32 => ui.mono_strong("32-bit FNV prime: 16777619\n Initial Value: 2166136261"),
            FnvVariant::L64 => ui.mono_strong("64-bit FNV prime: 1099511628211\n Initial Value: 14695981039346656037"),
            FnvVariant::L128 => ui.mono_strong("128-bit FNV prime: 309485009821345068724781371\n Initial Value: 144066263297769815596495629667062367629"),
            FnvVariant::L256 => ui.mono_strong(
                "256-bit FNV prime: 374144419156711147060143317175368453031918731002211\n Initial Value: 100029257958052580907070968620625704837092796014241193945225284501741471925557", 
            ),
            FnvVariant::L512 => ui.mono_strong(
                "512-bit FNV prime: 35835915874844867368919076489095108449946327955754392558399825615420669938882575126094039892345713852759\n Initial Value: 9659303129496669498009435400716310466090418745672637896108374329434462657994582932197716438449813051892206539805784495328239340083876191928701583869517785", 
            ),
            FnvVariant::L1024 => ui.mono_strong(
                "1024-bit FNV prime: 5016456510113118655434598811035278955030765345404790744303017523831112055108147451509157692220295382716162651878526895249385292291816524375083746691371804094271873160484737966720260389217684476157468082573\n Initial Value: 14197795064947621068722070641403218320880622795441933960878474914617582723252296732303717722150864096521202355549365628174669108571814760471015076148029755969804077320157692458563003215304957150157403644460363550505412711285966361610267868082893823963790439336411086884584107735010676915", 
            ),
        };

        ui.add_space(16.0);
    }

    fn hash_string(&self, text: &str) -> Result<String, hashers::errors::HasherError> {
        let bytes = self
            .input_format
            .text_to_bytes(text)
            .map_err(|_| hashers::errors::HasherError::general("byte format error"))?;

        let h = match self.variant {
            FnvVariant::L32 => Fnv32::init(self.alternate, self.zero_basis).update_and_finalize(&bytes),
            FnvVariant::L64 => Fnv64::init(self.alternate, self.zero_basis).update_and_finalize(&bytes),
            FnvVariant::L128 => Fnv128::init(self.alternate, self.zero_basis).update_and_finalize(&bytes),
            FnvVariant::L256 => Fnv256::init(self.alternate, self.zero_basis).update_and_finalize(&bytes),
            FnvVariant::L512 => Fnv512::init(self.alternate, self.zero_basis).update_and_finalize(&bytes),
            FnvVariant::L1024 => Fnv1024::init(self.alternate, self.zero_basis).update_and_finalize(&bytes),
        };

        Ok(self.output_format.byte_slice_to_text(&h))
    }
}
