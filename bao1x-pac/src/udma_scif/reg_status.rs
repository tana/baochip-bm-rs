#[doc = "Register `REG_STATUS` reader"]
pub type R = crate::R<RegStatusSpec>;
#[doc = "Register `REG_STATUS` writer"]
pub type W = crate::W<RegStatusSpec>;
#[doc = "Field `status_i` reader - status_i"]
pub type StatusIR = crate::BitReader;
#[doc = "Field `status_i` writer - status_i"]
pub type StatusIW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - status_i"]
    #[inline(always)]
    pub fn status_i(&self) -> StatusIR {
        StatusIR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - status_i"]
    #[inline(always)]
    pub fn status_i(&mut self) -> StatusIW<'_, RegStatusSpec> {
        StatusIW::new(self, 0)
    }
}
#[doc = "See `udma_scif_reg.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/ifsub/rtl/udma_scif_reg.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegStatusSpec;
impl crate::RegisterSpec for RegStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_status::R`](R) reader structure"]
impl crate::Readable for RegStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_status::W`](W) writer structure"]
impl crate::Writable for RegStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_STATUS to value 0"]
impl crate::Resettable for RegStatusSpec {}
