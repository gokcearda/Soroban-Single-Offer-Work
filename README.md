# soroban-single-offer-work

Bu proje, Soroban akıllı sözleşme platformu üzerinde geliştirilmiş, tek bir alım-satım teklifinin (single offer) oluşturulmasına ve yönetilmesine olanak tanıyan bir örnektir. Satıcı bir token çifti için belirli bir fiyattan teklif sunar, kontrata tokenlarını yatırır ve alıcılar bu teklife göre güvene dayalı olmayan (trustless) bir şekilde takas yapabilirler. Sözleşme ayrıca işlem geçmişi ve son işlem fiyatı gibi bazı temel verileri de saklar.

## Özellikler

-   **Tek Teklif:** Sözleşme başına yalnızca bir aktif alım-satım teklifi yönetilir.
-   **Satıcı Kontrolü:** Teklifi oluşturan satıcı, fiyatları, minimum alım miktarını güncelleyebilir ve teklifi aktif/pasif duruma getirebilir.
-   **Token Takası:** Alıcılar, satıcının belirlediği oran üzerinden token takası yapabilir.
-   **Likidite Yönetimi:** Satıcı, satmak istediği tokenları sözleşmeye yatırır ve satılmayan tokenları veya takas sonucu elde ettiği tokenları çekebilir.
-   **İşlem Geçmişi:** Son 10 işlemin kaydı tutulur.
-   **Son Fiyat Bilgisi:** Gerçekleşen son işlemin fiyat detayları sorgulanabilir.
-   **Minimum Alım Miktarı:** Satıcı, her bir işlem için minimum bir `buy_token` miktarı belirleyebilir.

## Ön Gereksinimler

-   Rust (en son stabil sürüm önerilir)
-   Cargo (Rust ile birlikte gelir)
-   Soroban CLI (sözleşmeyi deploy etmek ve ağ ile etkileşimde bulunmak için)

## Kurulum ve Başlangıç

Projeyi yerel makinenize klonlayın:

```bash
git clone https://github.com/gokcearda/Soroban-Single-Offer-Work.git
cd soroban-single-offer-work