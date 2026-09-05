#[doc = "Register `PERIPH_ID_1` reader"]
pub type R = crate::R<PeriphId1Spec>;
#[doc = "Register `PERIPH_ID_1` writer"]
pub type W = crate::W<PeriphId1Spec>;
#[doc = "Field `PART_NUMBER_MSB` reader - Identifies the part number"]
pub type PartNumberMsbR = crate::FieldReader;
#[doc = "Field `JEP106_LSB` reader - Designer ID LSB"]
pub type Jep106LsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Identifies the part number"]
    #[inline(always)]
    pub fn part_number_msb(&self) -> PartNumberMsbR {
        PartNumberMsbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Designer ID LSB"]
    #[inline(always)]
    pub fn jep106_lsb(&self) -> Jep106LsbR {
        Jep106LsbR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {}
#[doc = "Peripheral ID byte 1\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId1Spec;
impl crate::RegisterSpec for PeriphId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_1::R`](R) reader structure"]
impl crate::Readable for PeriphId1Spec {}
#[doc = "`write(|w| ..)` method takes [`periph_id_1::W`](W) writer structure"]
impl crate::Writable for PeriphId1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPH_ID_1 to value 0"]
impl crate::Resettable for PeriphId1Spec {}
