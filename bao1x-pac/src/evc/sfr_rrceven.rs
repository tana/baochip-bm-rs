#[doc = "Register `SFR_RRCEVEN` reader"]
pub type R = crate::R<SfrRrcevenSpec>;
#[doc = "Register `SFR_RRCEVEN` writer"]
pub type W = crate::W<SfrRrcevenSpec>;
#[doc = "Field `rrc_even` reader - rrc_even read/write control register"]
pub type RrcEvenR = crate::FieldReader<u16>;
#[doc = "Field `rrc_even` writer - rrc_even read/write control register"]
pub type RrcEvenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - rrc_even read/write control register"]
    #[inline(always)]
    pub fn rrc_even(&self) -> RrcEvenR {
        RrcEvenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - rrc_even read/write control register"]
    #[inline(always)]
    pub fn rrc_even(&mut self) -> RrcEvenW<'_, SfrRrcevenSpec> {
        RrcEvenW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L154 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L154>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrceven::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrceven::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcevenSpec;
impl crate::RegisterSpec for SfrRrcevenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrceven::R`](R) reader structure"]
impl crate::Readable for SfrRrcevenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrceven::W`](W) writer structure"]
impl crate::Writable for SfrRrcevenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCEVEN to value 0"]
impl crate::Resettable for SfrRrcevenSpec {}
