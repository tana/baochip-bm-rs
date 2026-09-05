#[doc = "Register `REG_PREFD0` reader"]
pub type R = crate::R<RegPrefd0Spec>;
#[doc = "Register `REG_PREFD0` writer"]
pub type W = crate::W<RegPrefd0Spec>;
#[doc = "Field `lsclk_prefd_0` reader - lsclk_prefd_0"]
pub type LsclkPrefd0R = crate::FieldReader<u16>;
#[doc = "Field `lsclk_prefd_0` writer - lsclk_prefd_0"]
pub type LsclkPrefd0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - lsclk_prefd_0"]
    #[inline(always)]
    pub fn lsclk_prefd_0(&self) -> LsclkPrefd0R {
        LsclkPrefd0R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - lsclk_prefd_0"]
    #[inline(always)]
    pub fn lsclk_prefd_0(&mut self) -> LsclkPrefd0W<'_, RegPrefd0Spec> {
        LsclkPrefd0W::new(self, 0)
    }
}
#[doc = "See `adv_timer_apb_if.sv <https://github.com/baochip/baochip-1x/blob/main/rtl/ip s/apb/apb_adv_timer/rtl/adv_timer_apb_if.sv>`__\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_prefd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_prefd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPrefd0Spec;
impl crate::RegisterSpec for RegPrefd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_prefd0::R`](R) reader structure"]
impl crate::Readable for RegPrefd0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_prefd0::W`](W) writer structure"]
impl crate::Writable for RegPrefd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PREFD0 to value 0"]
impl crate::Resettable for RegPrefd0Spec {}
