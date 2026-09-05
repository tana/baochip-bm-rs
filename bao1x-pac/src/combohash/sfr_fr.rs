#[doc = "Register `SFR_FR` reader"]
pub type R = crate::R<SfrFrSpec>;
#[doc = "Register `SFR_FR` writer"]
pub type W = crate::W<SfrFrSpec>;
#[doc = "Field `mfsm_done` reader - mfsm_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type MfsmDoneR = crate::BitReader;
#[doc = "Field `mfsm_done` writer - mfsm_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type MfsmDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hash_done` reader - hash_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type HashDoneR = crate::BitReader;
#[doc = "Field `hash_done` writer - hash_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type HashDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chnlo_done` reader - chnlo_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChnloDoneR = crate::BitReader;
#[doc = "Field `chnlo_done` writer - chnlo_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChnloDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chnli_done` reader - chnli_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChnliDoneR = crate::BitReader;
#[doc = "Field `chnli_done` writer - chnli_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChnliDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chkdone` reader - chkdone flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChkdoneR = crate::BitReader;
#[doc = "Field `chkdone` writer - chkdone flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChkdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chkpass` reader - chkpass flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChkpassR = crate::BitReader;
#[doc = "Field `chkpass` writer - chkpass flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChkpassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `chkfail` reader - chkfail flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChkfailR = crate::BitReader;
#[doc = "Field `chkfail` writer - chkfail flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type ChkfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - mfsm_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn mfsm_done(&self) -> MfsmDoneR {
        MfsmDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hash_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn hash_done(&self) -> HashDoneR {
        HashDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - chnlo_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chnlo_done(&self) -> ChnloDoneR {
        ChnloDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - chnli_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chnli_done(&self) -> ChnliDoneR {
        ChnliDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - chkdone flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chkdone(&self) -> ChkdoneR {
        ChkdoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - chkpass flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chkpass(&self) -> ChkpassR {
        ChkpassR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - chkfail flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chkfail(&self) -> ChkfailR {
        ChkfailR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mfsm_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn mfsm_done(&mut self) -> MfsmDoneW<'_, SfrFrSpec> {
        MfsmDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - hash_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn hash_done(&mut self) -> HashDoneW<'_, SfrFrSpec> {
        HashDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - chnlo_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chnlo_done(&mut self) -> ChnloDoneW<'_, SfrFrSpec> {
        ChnloDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - chnli_done flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chnli_done(&mut self) -> ChnliDoneW<'_, SfrFrSpec> {
        ChnliDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - chkdone flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chkdone(&mut self) -> ChkdoneW<'_, SfrFrSpec> {
        ChkdoneW::new(self, 4)
    }
    #[doc = "Bit 5 - chkpass flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chkpass(&mut self) -> ChkpassW<'_, SfrFrSpec> {
        ChkpassW::new(self, 5)
    }
    #[doc = "Bit 6 - chkfail flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn chkfail(&mut self) -> ChkfailW<'_, SfrFrSpec> {
        ChkfailW::new(self, 6)
    }
}
#[doc = "See `combohasha.sv#L211 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L211>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFrSpec;
impl crate::RegisterSpec for SfrFrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_fr::R`](R) reader structure"]
impl crate::Readable for SfrFrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_fr::W`](W) writer structure"]
impl crate::Writable for SfrFrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FR to value 0"]
impl crate::Resettable for SfrFrSpec {}
