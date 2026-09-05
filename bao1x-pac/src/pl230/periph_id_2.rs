#[doc = "Register `PERIPH_ID_2` reader"]
pub type R = crate::R<PeriphId2Spec>;
#[doc = "Register `PERIPH_ID_2` writer"]
pub type W = crate::W<PeriphId2Spec>;
#[doc = "Field `JEP106_MSB` reader - Designer ID MSB"]
pub type Jep106MsbR = crate::FieldReader;
#[doc = "Field `JEDEC_USED` reader - Identifies if JP106 ID code is used"]
pub type JedecUsedR = crate::BitReader;
#[doc = "Field `REVISION` reader - Identifies revision number of peripheral"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Designer ID MSB"]
    #[inline(always)]
    pub fn jep106_msb(&self) -> Jep106MsbR {
        Jep106MsbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Identifies if JP106 ID code is used"]
    #[inline(always)]
    pub fn jedec_used(&self) -> JedecUsedR {
        JedecUsedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Identifies revision number of peripheral"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Peripheral ID byte 2\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId2Spec;
impl crate::RegisterSpec for PeriphId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_2::R`](R) reader structure"]
impl crate::Readable for PeriphId2Spec {}
#[doc = "`write(|w| ..)` method takes [`periph_id_2::W`](W) writer structure"]
impl crate::Writable for PeriphId2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPH_ID_2 to value 0"]
impl crate::Resettable for PeriphId2Spec {}
