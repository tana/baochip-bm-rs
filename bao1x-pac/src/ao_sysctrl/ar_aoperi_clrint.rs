#[doc = "Register `AR_AOPERI_CLRINT` reader"]
pub type R = crate::R<ArAoperiClrintSpec>;
#[doc = "Register `AR_AOPERI_CLRINT` writer"]
pub type W = crate::W<ArAoperiClrintSpec>;
#[doc = "Field `ar_aoperi_clrint` reader - ar_aoperi_clrint performs action on write of value: 0xaa"]
pub type ArAoperiClrintR = crate::FieldReader<u32>;
#[doc = "Field `ar_aoperi_clrint` writer - ar_aoperi_clrint performs action on write of value: 0xaa"]
pub type ArAoperiClrintW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ar_aoperi_clrint performs action on write of value: 0xaa"]
    #[inline(always)]
    pub fn ar_aoperi_clrint(&self) -> ArAoperiClrintR {
        ArAoperiClrintR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ar_aoperi_clrint performs action on write of value: 0xaa"]
    #[inline(always)]
    pub fn ar_aoperi_clrint(&mut self) -> ArAoperiClrintW<'_, ArAoperiClrintSpec> {
        ArAoperiClrintW::new(self, 0)
    }
}
#[doc = "See `ao_sysctrl.sv#L393 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/ao/rtl/ao_sysctrl.sv#L393>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`ar_aoperi_clrint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_aoperi_clrint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArAoperiClrintSpec;
impl crate::RegisterSpec for ArAoperiClrintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar_aoperi_clrint::R`](R) reader structure"]
impl crate::Readable for ArAoperiClrintSpec {}
#[doc = "`write(|w| ..)` method takes [`ar_aoperi_clrint::W`](W) writer structure"]
impl crate::Writable for ArAoperiClrintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AR_AOPERI_CLRINT to value 0"]
impl crate::Resettable for ArAoperiClrintSpec {}
