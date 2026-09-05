#[doc = "Register `REG_CH_EN` reader"]
pub type R = crate::R<RegChEnSpec>;
#[doc = "Register `REG_CH_EN` writer"]
pub type W = crate::W<RegChEnSpec>;
#[doc = "Field `r_clk_en` reader - r_clk_en"]
pub type RClkEnR = crate::FieldReader;
#[doc = "Field `r_clk_en` writer - r_clk_en"]
pub type RClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - r_clk_en"]
    #[inline(always)]
    pub fn r_clk_en(&self) -> RClkEnR {
        RClkEnR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - r_clk_en"]
    #[inline(always)]
    pub fn r_clk_en(&mut self) -> RClkEnW<'_, RegChEnSpec> {
        RClkEnW::new(self, 0)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ch_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ch_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChEnSpec;
impl crate::RegisterSpec for RegChEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ch_en::R`](R) reader structure"]
impl crate::Readable for RegChEnSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ch_en::W`](W) writer structure"]
impl crate::Writable for RegChEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CH_EN to value 0"]
impl crate::Resettable for RegChEnSpec {}
