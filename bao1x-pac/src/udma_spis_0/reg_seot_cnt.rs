#[doc = "Register `REG_SEOT_CNT` reader"]
pub type R = crate::R<RegSeotCntSpec>;
#[doc = "Register `REG_SEOT_CNT` writer"]
pub type W = crate::W<RegSeotCntSpec>;
#[doc = "Field `sr_seot_cnt` reader - sr_seot_cnt"]
pub type SrSeotCntR = crate::FieldReader<u16>;
#[doc = "Field `sr_seot_cnt` writer - sr_seot_cnt"]
pub type SrSeotCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sr_seot_cnt"]
    #[inline(always)]
    pub fn sr_seot_cnt(&self) -> SrSeotCntR {
        SrSeotCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sr_seot_cnt"]
    #[inline(always)]
    pub fn sr_seot_cnt(&mut self) -> SrSeotCntW<'_, RegSeotCntSpec> {
        SrSeotCntW::new(self, 0)
    }
}
#[doc = "See `udma_spis_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_spis_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_seot_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_seot_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSeotCntSpec;
impl crate::RegisterSpec for RegSeotCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_seot_cnt::R`](R) reader structure"]
impl crate::Readable for RegSeotCntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_seot_cnt::W`](W) writer structure"]
impl crate::Writable for RegSeotCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SEOT_CNT to value 0"]
impl crate::Resettable for RegSeotCntSpec {}
