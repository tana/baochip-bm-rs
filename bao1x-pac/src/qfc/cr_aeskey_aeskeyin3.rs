#[doc = "Register `CR_AESKEY_AESKEYIN3` reader"]
pub type R = crate::R<CrAeskeyAeskeyin3Spec>;
#[doc = "Register `CR_AESKEY_AESKEYIN3` writer"]
pub type W = crate::W<CrAeskeyAeskeyin3Spec>;
#[doc = "Field `aeskeyin3` reader - cr_aeskey read/write control register"]
pub type Aeskeyin3R = crate::FieldReader<u32>;
#[doc = "Field `aeskeyin3` writer - cr_aeskey read/write control register"]
pub type Aeskeyin3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_aeskey read/write control register"]
    #[inline(always)]
    pub fn aeskeyin3(&self) -> Aeskeyin3R {
        Aeskeyin3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_aeskey read/write control register"]
    #[inline(always)]
    pub fn aeskeyin3(&mut self) -> Aeskeyin3W<'_, CrAeskeyAeskeyin3Spec> {
        Aeskeyin3W::new(self, 0)
    }
}
#[doc = "See `qfc.sv#L200 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/co re/rtl/qfc.sv#L200>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_aeskey_aeskeyin3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_aeskey_aeskeyin3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrAeskeyAeskeyin3Spec;
impl crate::RegisterSpec for CrAeskeyAeskeyin3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_aeskey_aeskeyin3::R`](R) reader structure"]
impl crate::Readable for CrAeskeyAeskeyin3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_aeskey_aeskeyin3::W`](W) writer structure"]
impl crate::Writable for CrAeskeyAeskeyin3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_AESKEY_AESKEYIN3 to value 0"]
impl crate::Resettable for CrAeskeyAeskeyin3Spec {}
