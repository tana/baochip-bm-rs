#[doc = "Register `REG_PREFD1` reader"]
pub type R = crate::R<RegPrefd1Spec>;
#[doc = "Register `REG_PREFD1` writer"]
pub type W = crate::W<RegPrefd1Spec>;
#[doc = "Field `lsclk_prefd_1` reader - lsclk_prefd_1"]
pub type LsclkPrefd1R = crate::FieldReader<u16>;
#[doc = "Field `lsclk_prefd_1` writer - lsclk_prefd_1"]
pub type LsclkPrefd1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - lsclk_prefd_1"]
    #[inline(always)]
    pub fn lsclk_prefd_1(&self) -> LsclkPrefd1R {
        LsclkPrefd1R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - lsclk_prefd_1"]
    #[inline(always)]
    pub fn lsclk_prefd_1(&mut self) -> LsclkPrefd1W<'_, RegPrefd1Spec> {
        LsclkPrefd1W::new(self, 0)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPrefd1Spec;
impl crate::RegisterSpec for RegPrefd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_prefd1::R`](R) reader structure"]
impl crate::Readable for RegPrefd1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_prefd1::W`](W) writer structure"]
impl crate::Writable for RegPrefd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PREFD1 to value 0"]
impl crate::Resettable for RegPrefd1Spec {}
