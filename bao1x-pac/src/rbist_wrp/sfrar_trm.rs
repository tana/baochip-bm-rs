#[doc = "Register `SFRAR_TRM` reader"]
pub type R = crate::R<SfrarTrmSpec>;
#[doc = "Register `SFRAR_TRM` writer"]
pub type W = crate::W<SfrarTrmSpec>;
#[doc = "Field `sfrar_trm` reader - sfrar_trm performs action on write of value: 0x5a"]
pub type SfrarTrmR = crate::FieldReader<u32>;
#[doc = "Field `sfrar_trm` writer - sfrar_trm performs action on write of value: 0x5a"]
pub type SfrarTrmW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfrar_trm performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfrar_trm(&self) -> SfrarTrmR {
        SfrarTrmR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfrar_trm performs action on write of value: 0x5a"]
    #[inline(always)]
    pub fn sfrar_trm(&mut self) -> SfrarTrmW<'_, SfrarTrmSpec> {
        SfrarTrmW::new(self, 0)
    }
}
#[doc = "See `rbist_wrp.sv#L176 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L176>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrar_trm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrar_trm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrarTrmSpec;
impl crate::RegisterSpec for SfrarTrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfrar_trm::R`](R) reader structure"]
impl crate::Readable for SfrarTrmSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrar_trm::W`](W) writer structure"]
impl crate::Writable for SfrarTrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRAR_TRM to value 0"]
impl crate::Resettable for SfrarTrmSpec {}
