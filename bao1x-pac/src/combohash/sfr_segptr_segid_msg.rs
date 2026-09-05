#[doc = "Register `SFR_SEGPTR_SEGID_MSG` reader"]
pub type R = crate::R<SfrSegptrSegidMsgSpec>;
#[doc = "Register `SFR_SEGPTR_SEGID_MSG` writer"]
pub type W = crate::W<SfrSegptrSegidMsgSpec>;
#[doc = "Field `SEGID_MSG` reader - cr_segptrstart read/write control register"]
pub type SegidMsgR = crate::FieldReader<u16>;
#[doc = "Field `SEGID_MSG` writer - cr_segptrstart read/write control register"]
pub type SegidMsgW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_msg(&self) -> SegidMsgR {
        SegidMsgR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - cr_segptrstart read/write control register"]
    #[inline(always)]
    pub fn segid_msg(&mut self) -> SegidMsgW<'_, SfrSegptrSegidMsgSpec> {
        SegidMsgW::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L219 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L219>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_segid_msg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_segid_msg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrSegidMsgSpec;
impl crate::RegisterSpec for SfrSegptrSegidMsgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_segid_msg::R`](R) reader structure"]
impl crate::Readable for SfrSegptrSegidMsgSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_segid_msg::W`](W) writer structure"]
impl crate::Writable for SfrSegptrSegidMsgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_SEGID_MSG to value 0"]
impl crate::Resettable for SfrSegptrSegidMsgSpec {}
