#[doc = "Register `SFR_SRMFSM` reader"]
pub type R = crate::R<SfrSrmfsmSpec>;
#[doc = "Register `SFR_SRMFSM` writer"]
pub type W = crate::W<SfrSrmfsmSpec>;
#[doc = "Field `mfsm` reader - mfsm read only status register"]
pub type MfsmR = crate::FieldReader;
#[doc = "Field `mfsm` writer - mfsm read only status register"]
pub type MfsmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `crreg` reader - crreg read only status register"]
pub type CrregR = crate::BitReader;
#[doc = "Field `crreg` writer - crreg read only status register"]
pub type CrregW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - mfsm read only status register"]
    #[inline(always)]
    pub fn mfsm(&self) -> MfsmR {
        MfsmR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - crreg read only status register"]
    #[inline(always)]
    pub fn crreg(&self) -> CrregR {
        CrregR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - mfsm read only status register"]
    #[inline(always)]
    pub fn mfsm(&mut self) -> MfsmW<'_, SfrSrmfsmSpec> {
        MfsmW::new(self, 0)
    }
    #[doc = "Bit 8 - crreg read only status register"]
    #[inline(always)]
    pub fn crreg(&mut self) -> CrregW<'_, SfrSrmfsmSpec> {
        CrregW::new(self, 8)
    }
}
#[doc = "See `alu.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L138>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_srmfsm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_srmfsm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrmfsmSpec;
impl crate::RegisterSpec for SfrSrmfsmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_srmfsm::R`](R) reader structure"]
impl crate::Readable for SfrSrmfsmSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_srmfsm::W`](W) writer structure"]
impl crate::Writable for SfrSrmfsmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SRMFSM to value 0"]
impl crate::Resettable for SfrSrmfsmSpec {}
