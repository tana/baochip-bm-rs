#[doc = "Register `PERIPH_ID_0` reader"]
pub type R = crate::R<PeriphId0Spec>;
#[doc = "Register `PERIPH_ID_0` writer"]
pub type W = crate::W<PeriphId0Spec>;
#[doc = "Field `PART_NUMBER_LSB` reader - Identifies the part number"]
pub type PartNumberLsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Identifies the part number"]
    #[inline(always)]
    pub fn part_number_lsb(&self) -> PartNumberLsbR {
        PartNumberLsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Peripheral ID byte 0\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_id_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periph_id_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId0Spec;
impl crate::RegisterSpec for PeriphId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id_0::R`](R) reader structure"]
impl crate::Readable for PeriphId0Spec {}
#[doc = "`write(|w| ..)` method takes [`periph_id_0::W`](W) writer structure"]
impl crate::Writable for PeriphId0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIPH_ID_0 to value 0"]
impl crate::Resettable for PeriphId0Spec {}
