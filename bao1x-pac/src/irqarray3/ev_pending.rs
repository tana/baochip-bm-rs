#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `trng_done` reader - `1` when a \"trng_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type TrngDoneR = crate::BitReader;
#[doc = "Field `trng_done` writer - `1` when a \"trng_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type TrngDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_done` reader - `1` when a \"aes_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AesDoneR = crate::BitReader;
#[doc = "Field `aes_done` writer - `1` when a \"aes_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AesDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pke_done` reader - `1` when a \"pke_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type PkeDoneR = crate::BitReader;
#[doc = "Field `pke_done` writer - `1` when a \"pke_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type PkeDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hash_done` reader - `1` when a \"hash_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type HashDoneR = crate::BitReader;
#[doc = "Field `hash_done` writer - `1` when a \"hash_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type HashDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `alu_done` reader - `1` when a \"alu_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AluDoneR = crate::BitReader;
#[doc = "Field `alu_done` writer - `1` when a \"alu_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AluDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_ichdone` reader - `1` when a \"sdma_ichdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdmaIchdoneR = crate::BitReader;
#[doc = "Field `sdma_ichdone` writer - `1` when a \"sdma_ichdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdmaIchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_schdone` reader - `1` when a \"sdma_schdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdmaSchdoneR = crate::BitReader;
#[doc = "Field `sdma_schdone` writer - `1` when a \"sdma_schdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdmaSchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdma_xchdone` reader - `1` when a \"sdma_xchdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdmaXchdoneR = crate::BitReader;
#[doc = "Field `sdma_xchdone` writer - `1` when a \"sdma_xchdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdmaXchdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s8` reader - `1` when a \"nc_b3s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s8R = crate::BitReader;
#[doc = "Field `nc_b3s8` writer - `1` when a \"nc_b3s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s9` reader - `1` when a \"nc_b3s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s9R = crate::BitReader;
#[doc = "Field `nc_b3s9` writer - `1` when a \"nc_b3s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s10` reader - `1` when a \"nc_b3s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s10R = crate::BitReader;
#[doc = "Field `nc_b3s10` writer - `1` when a \"nc_b3s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s11` reader - `1` when a \"nc_b3s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s11R = crate::BitReader;
#[doc = "Field `nc_b3s11` writer - `1` when a \"nc_b3s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s12` reader - `1` when a \"nc_b3s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s12R = crate::BitReader;
#[doc = "Field `nc_b3s12` writer - `1` when a \"nc_b3s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s13` reader - `1` when a \"nc_b3s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s13R = crate::BitReader;
#[doc = "Field `nc_b3s13` writer - `1` when a \"nc_b3s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s14` reader - `1` when a \"nc_b3s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s14R = crate::BitReader;
#[doc = "Field `nc_b3s14` writer - `1` when a \"nc_b3s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b3s15` reader - `1` when a \"nc_b3s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s15R = crate::BitReader;
#[doc = "Field `nc_b3s15` writer - `1` when a \"nc_b3s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB3s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` when a \"trng_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn trng_done(&self) -> TrngDoneR {
        TrngDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `1` when a \"aes_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn aes_done(&self) -> AesDoneR {
        AesDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - `1` when a \"pke_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pke_done(&self) -> PkeDoneR {
        PkeDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `1` when a \"hash_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn hash_done(&self) -> HashDoneR {
        HashDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `1` when a \"alu_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn alu_done(&self) -> AluDoneR {
        AluDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `1` when a \"sdma_ichdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdma_ichdone(&self) -> SdmaIchdoneR {
        SdmaIchdoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - `1` when a \"sdma_schdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdma_schdone(&self) -> SdmaSchdoneR {
        SdmaSchdoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - `1` when a \"sdma_xchdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdma_xchdone(&self) -> SdmaXchdoneR {
        SdmaXchdoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `1` when a \"nc_b3s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s8(&self) -> NcB3s8R {
        NcB3s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - `1` when a \"nc_b3s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s9(&self) -> NcB3s9R {
        NcB3s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b3s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s10(&self) -> NcB3s10R {
        NcB3s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b3s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s11(&self) -> NcB3s11R {
        NcB3s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b3s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s12(&self) -> NcB3s12R {
        NcB3s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b3s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s13(&self) -> NcB3s13R {
        NcB3s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b3s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s14(&self) -> NcB3s14R {
        NcB3s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b3s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s15(&self) -> NcB3s15R {
        NcB3s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` when a \"trng_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn trng_done(&mut self) -> TrngDoneW<'_, EvPendingSpec> {
        TrngDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - `1` when a \"aes_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn aes_done(&mut self) -> AesDoneW<'_, EvPendingSpec> {
        AesDoneW::new(self, 1)
    }
    #[doc = "Bit 2 - `1` when a \"pke_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pke_done(&mut self) -> PkeDoneW<'_, EvPendingSpec> {
        PkeDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - `1` when a \"hash_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn hash_done(&mut self) -> HashDoneW<'_, EvPendingSpec> {
        HashDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - `1` when a \"alu_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn alu_done(&mut self) -> AluDoneW<'_, EvPendingSpec> {
        AluDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - `1` when a \"sdma_ichdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdma_ichdone(&mut self) -> SdmaIchdoneW<'_, EvPendingSpec> {
        SdmaIchdoneW::new(self, 5)
    }
    #[doc = "Bit 6 - `1` when a \"sdma_schdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdma_schdone(&mut self) -> SdmaSchdoneW<'_, EvPendingSpec> {
        SdmaSchdoneW::new(self, 6)
    }
    #[doc = "Bit 7 - `1` when a \"sdma_xchdone\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdma_xchdone(&mut self) -> SdmaXchdoneW<'_, EvPendingSpec> {
        SdmaXchdoneW::new(self, 7)
    }
    #[doc = "Bit 8 - `1` when a \"nc_b3s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s8(&mut self) -> NcB3s8W<'_, EvPendingSpec> {
        NcB3s8W::new(self, 8)
    }
    #[doc = "Bit 9 - `1` when a \"nc_b3s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s9(&mut self) -> NcB3s9W<'_, EvPendingSpec> {
        NcB3s9W::new(self, 9)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b3s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s10(&mut self) -> NcB3s10W<'_, EvPendingSpec> {
        NcB3s10W::new(self, 10)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b3s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s11(&mut self) -> NcB3s11W<'_, EvPendingSpec> {
        NcB3s11W::new(self, 11)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b3s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s12(&mut self) -> NcB3s12W<'_, EvPendingSpec> {
        NcB3s12W::new(self, 12)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b3s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s13(&mut self) -> NcB3s13W<'_, EvPendingSpec> {
        NcB3s13W::new(self, 13)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b3s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s14(&mut self) -> NcB3s14W<'_, EvPendingSpec> {
        NcB3s14W::new(self, 14)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b3s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b3s15(&mut self) -> NcB3s15W<'_, EvPendingSpec> {
        NcB3s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b3s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvPendingSpec;
impl crate::RegisterSpec for EvPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EvPendingSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EvPendingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_PENDING to value 0"]
impl crate::Resettable for EvPendingSpec {}
