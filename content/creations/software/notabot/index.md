+++
title = "Not-a-Bot"
date = 2020-12-01
description = "Proof of unique passport identity system"
card = "artefacts/notabot.jpg"
+++

Not-A-Bot was a proof-of-passport identity solution, developed in collaboration with Philip Sheldrake and the Akasha Foundation. The idea being to simply prove uniqueness through institutionally accepted means in order to bootstrap decentrailsed identity from a recognisable trust anchor. This relates to work carried out for the Dutch Blockchain Coalition. 

---

<!-- 

TODO: Add thumbnail
TODO: Remove transclusion headers in Obsidian
TODO: Compile and add project documentation

-->

# Not-A-Bot: Overview
## About
NotABot functions as a sybil proof identity system; specifically proof-of-citizenship. It enables people to pseudonymously prove their accounts stem from a unique root identifier, e-passports, without leaking any information to third parties.

## Mechanism
Alice scans her e-passport with her phones NFC reader. The hash of this data is compared against the Civil Aviation Organisations public key directory. If it is a real passport the check passes, if it is lost, stolen, expired or non-existant it fails.

Assuming the check passes, the passport hash is added to the NotABot whitelist (SaaS for now) and can be used as a seed to generate user accounts with third party services verifying that Alice is very likely human. This hash can be used to generate multiple accounts which we do not limit, but third party services can arbitrarily cap the number of allowable accounts within their own ecosystems.

For more sensitive matters like pseudonymous digital democracies a proof-of-uniqueness credential is available; Alice simply nominates one of her personas within a given system as unique, which she can only do once.

## Benefits
Reduced pollution of the digital information ecology as bots and fake accounts on social media are throttled.
Trustworthy pseudonymous digital democracy enabled for the first time.

## Business models

### API Calls
Charge third party services for each API call. This is akin to "utility token" models.

### Evangelists
Evangilists pay a small amount (0.99¢) on the app store to obtain a number of proofs that they can then distribute to others.

### Membership
Members pay a small amount (0.99¢) on the app store to obtain a single proof for themselves which pays for our operations. Excess capital can be democratically allocated to causes via Coop/DAO functionality.

### Goverment/Philantropy
Apply for govermental/philantropic grants. In lucy circumstances the war on disinformation may become more politicised and thus goverments may wish to sponsor its citizens for a small amount. Either case gets PR value from a logo on our website.

### Bolt-on Services
Support, customisation, third party integration, GDPR consulting, PR etc.

---

# About Not-A-Bot
Governments go to great and expensive lengths to design for administration by and of (unique and human) citizens. Over 150 states have issued more than 1 billion [ePassports])(https://en.wikipedia.org/wiki/Biometric_passport) [ref](https://www.thalesgroup.com/en/markets/digital-identity-and-security/government/passport/electronic-passport-trends), and so this seems to be the obvious infrastructure to wield here if we do so with due care and attention.

ePassports are distributed technology — a border control officer can validate any ePassport without needing an internet connection. If the ePassport is fake then it won't be digitally signed by the member countries of the International Civil Aviation Organization Public Key Directory (see the [org's public key directory](https://download.pkd.icao.int/)).

However, ePassports are pregnant with potential for dystopian (degenerative) application, not least in the proliferation of KYC-as-a-service from providers who couldn't care less about emergent consequences. By intervening here, we amplify a 'power' of ePassports while taking a market-defining stance in protecting privacy and providing for [generativity](https://generative-identity.org/).

NOT-A-BOT leans lightly on ePassports, by which we mean it is specifically NOT about identification; it is definitely NOT about legal identity. Importantly, NOT-A-BOT does NOT confirm Alice is using her own ePassport, and although Alice’s phone reads the information stored on the ePassport, that’s as far as that information will ever go.

We benefit from deeply ingrained social norms that prevent the vast majority of people from even considering using someone else’s ePassport or lending someone else their own, combined with the difficulty of getting hold of other people’s ePassports without their permission and the system penalty here of being caught. These subtleties are significant to our ethos and service design. Remember, we’re not trying to ‘solve’ for any identity related problem here, just creating proofs.

NOT-A-BOT requires a smartphone app that can read and verify an ePassport, and check it is not counterfeit or expired. [Some open source code exists](https://github.com/Glamdring/epassport-reader). This video shows InnoValor's proprietary [ReadID](https://readid.com/) app in action.

# Registration Story
For the vast majority of those people wishing to prove humanness and uniqueness with NOT-A-BOT, the following two sections (Humanness and Uniqueness) describe the entirety of their 2-minute registration experience. A description of the exceptions to this process and our corresponding defenses is provided in Appendix A. This doc concludes with the non-profit business model.

## Humanness
Alice has joined the Ethereum World online community. Her profile offers the option to prove her humanness at any point, a characteristic that fellow members will see confirmed in the UI. She learns that this helps improve her experience of the network and the health of the network overall (no dishonest bots!) requiring nothing but a couple of minutes of her time and zero privacy risk. She can see others have already done so, and being community spirited and curious she decides to do so herself.

Alice downloads the NOT-A-BOT app and sees that no personally identifiable information (PII) is ever sent to the app developer (e.g. see 'User Privacy and Data Use' for iOS 14). She follows the app's instructions. The app confirms that Alice has a real, unexpired ePassport in her possession, and that's all. It does not confirm that it is Alice's and so does not need expensive machine or machine-assisted facial recognition. No personal data leaves her phone, only encrypted hashes of some ePassport data. The app stores the expiry date locally so it can remind Alice to get a new ePassport at the appropriate time.

Ethereum World calls the NOT-A-BOT API to show a QR code for Alice to scan with her NOT-A-BOT app. The app tells Alice that Ethereum World is asking to confirm her humanness (and not her uniqueness). She is reminded that:

this does not prevent her from doing so anywhere else, including additional accounts with Ethereum World
correlation is impossible
no personal information whatsoever is transferred to Ethereum World or in fact any party including the NOT-A-BOT service providers.
She hits the green ✓ button. All done. Her Ethereum World profile now displays: very likely human!

Well she's not quite done. Alice is required to scan the same ePassport again in 30 days just to confirm that it wasn't someone else's ePassport temporarily in her possession.

It is very unlikely that Alice will ever wish to maintain very many accounts with proof-of-humanness in any one system / domain. The NOT-A-BOT API therefore allows platforms / networks / communities to set a cap on the number. A cap fully accommodates generativity (involving our freedom to adopt different personae in different contexts) while remaining far too small to be of interest to those with nefarious purposes.

## Uniqueness
Alice now maintains two profiles on Ethereum World — one professional and one pseudonymous. Both are confirmed: very likely human!

She learns that governance of Ethereum World is decentralized and democratic, and that this may include the distribution of monies in the future. She wants to take part and appreciates the requirement to provide proof-of-uniqueness. She opts to present her pseudonymous profile as unique in this associated domain. To qualify, she repeats the process above for her pseudonymous profile but this time the app points out that the Ethereum World Association is asking for evidence of uniqueness. She hits the green ✓ button and her pseudonymous profile is accepted into governance membership: one and only!

Should she try, Alice will find that she cannot repeat the uniqueness process in this system / domain for her professional profile, at least that is without first removing the attribute for her pseudonymous profile.

## An ongoing process
Proof-of-humanness and proof-of-uniqueness is an ongoing process. For example, any proof that an ethereum address represents a human will require regular re-confirmation in order to prevent the emergence of a market in 'human ethereum addresses'.

# Other projects (with similar goals)
The [BrightID](http://brightid.org/) project sets out to ascertain human uniqueness (it isn’t designed to accommodate humanness without assertion of uniqueness), but such Web of Trust approaches have significant adoption challenges. One can envisage BrightID working at scale, just not how to achieve that scale in the first place. NOT-A-BOT can scale much more readily and then empower projects such as BrightID in the longer-term to go further than we can; not everyone has an ePassport after all.

Other approaches rely on KYC processes and necessarily involve personal information, some of which will be considered private in many contexts. KYC-based proofs may be traced to the person by malicious technical or wholly legal means, or simply as and when the individual concerned does not appreciate the personal or societal implications of disclosing more than he/she might otherwise. Bringing legal identity into the question of humanness is degenerative (see [The Dystopia of SSI](https://generative-identity.org/the-dystopia-of-self-sovereign-identity-ssi/)).

NOT-A-BOT avoids the significant complications innate to Web of Trust approaches, avoids the degenerative implications of KYC processes, and may even be said to 'externalise' key recovery, i.e. it falls to Alice and her government to sort out.

# Design Constraints
## Top-line technical design criteria
1. Confirm that Alice has access to an ePassport; no need to check if it's hers.
2. Make sure if Alice uses her Dutch ePassport her chances of also getting away with using her Swiss ePassport are minimised (per threat model in Appendix A).
3. Allow Alice to prove humanness or uniqueness as required without imparting any personal information, and ideally making sure she's only asked for the former unless the latter is critical in terms of, for example, democracy or wealth distribution.
4. Allow any platform / network / community to cap the number of accounts for which any one individual can provide proof of humanness.
5. Prevent the platform / network / community from correlating Alice, based on accounts running through not-a-bot services.
6. Ensure a 1:1 pairing of smartphone to ePassport (i.e. the same ePassport can never be used on multiple devices concurrently).
7. Allow Alice to change her smartphone at her discretion or because her last one is lost or stolen.
8. Allow Alice to transition from one ePassport to another.
9. Allow Alice to resume using the NOT-A-BOT service even if her smartphone and ePassport are lost or stolen (per threat model in Appendix A).
10. Allow Alice to use her own ePassport even if it has been used by someone else previously (per threat model in Appendix A).
11. Enable revalidation by the app to frustrate a market in proven accounts forming, and to check Alice is still alive of course.
12. Enable revalidation by ePassport (e.g. requiring Alice to go through the ePassport process again after 30 days and as required).

# Business model
## Income

We have explored five potential income sources, and have dismissed two.

### Dismissed options

#### The platform / network / community  - API consumers
Adding to their costs increases barriers-to-entry in a market and so frustrates competition from new players
If we limit API fees to just the big platforms, we get ‘owned’ effectively in the same way some consider Mozilla to be ‘owned’ by Google, and our future financial sustainability is contingent on their future financial sustainability.

#### The ad networks
If we work with the ad networks — who might be attracted to our service to demonstrably reduce click-fraud — then we may be seen to be in cahoots with the surveillance capitalists and helping to propagate advertising as the dominant business model on the web.
To be further explored

#### Membership
There is something very attractive about very many small contributions from the actual beneficiaries here — each and every one of us. Assuming that the necessary usability and accessibility of crypto remains several years out, the Apple and Android app stores remain the primary sales channel.

The app stores’ lowest price point is US$0.99, or the approximate price parity equivalent. For many, this may be more money than they would wish or might afford to spend on something like this.

We propose a NOT-A-BOT cooperative. Anyone can use NOT-A-BOT for free and the app invites you to optionally join the cooperative for $0.99 (or your local equivalent) per annum. The coop membership fees cover NOT-A-BOT operations, and the surplus is invested in good projects as determined democratically by membership.

Under the terms and conditions of service, those platforms / networks / communities that make use of NOT-A-BOT would be required to display status icons. The icons can communicate cooperative membership in order to help accrue social value to cooperative membership. For example:

#### Government
We can imagine the Dutch government for example embracing NOT-A-BOT as compatible with its vision and values and deciding to sponsor NOT-A-BOT to the tune of €0.25 per citizen per annum. Its sponsorship is recognized on our website and in the app. Anyone using a Dutch ePassport is automatically enrolled in the NOT-A-BOT cooperative.

#### SaaS
We are of course champions of the dweb. However, we haven't found a technical architecture to deliver the design criteria in a decentralized manner. On one hand this is frustrating, and on the other hand it does allow us to defend the SaaS system design which in turn means we can bring this to market more quickly and make it immediately more accessible to more people.

Importantly, unlike services premised on KYC, there is nothing to gain from hacking our system — the data we hold has no value to anyone else.

#### Philanthropy
We apply for philanthropic grants.

<!---

TODO: Mission Model Canvas
TODO: Business Model Canvas

--->

# Not-A-Bot: Technical specs
## Minimum Lovable Product
### Implementation
To be hosted in https://github.com/really-not-a-bot .

### Server-Side (Backend)
Small Python backend using [FastAPI](https://fastapi.tiangolo.com/) as web framework and [Redis](https://redis.io/) for storage. It will validate passport ids, store generated passport hashes and allow flagging them as bot.

### Data to handle:
Passport hashes.
Passport hash metadata (i.e. creation date, expiration date, etc.).
Passport hash attributes (i.e. not a bot, etc.).

### Endpoints:
Register new passport hash.
Flag passport hash as acting like a bot.
Check if passport hash was reported as acting like a bot.

### Certificates:
Public keys of all passport-granting authorities.

## Mobile Application (Frontend)
Cross-platform mobile application using [React Native](https://reactnative.dev/).

Application capable of undertaking user’s passport details, sending the data and receiving the response to & from the backend using API endpoints.
Generate a QR code for further integration and linking.
Temporarily store the data in a local instance by generating a file with key value pairs in both IoS & Android Application.

## Requirements:
[iOS developer account](https://developer.apple.com/programs/)
[Play Store developer account](https://play.google.com/console/u/0/signup)

## Infrastructure
Hosting & Continuous Delivery - (e.g. Heroku)

# E-passport verification app requirements
## Business requirements
### Access to app markets
#### iOS
Apple developers account:

- D-U-N-S® Number
- Legal entity name
- Headquarters address
- Mailing address
- Your work contact information
- Legal Entity Status
- Legal Binding Authority
- A Website

#### Android
- Google Play developer account (one off $25 fee)
- Developer name
- Company Google account
- Physical address (if payments required)
- [Google merchant account (?)](https://support.google.com/googleplay/android-developer/answer/3092739?hl=en)

## Legal & Tax
### [Tax strategy](https://support.google.com/paymentscenter/answer/7158952)

Legal docs regional compliance:
- Compliance with EU laws on document handling
- GDPR & privacy statement
- USA market?

### [ISO27001 certification (?)](https://www.itgovernance.co.uk/blog/requirements-for-achieving-iso-27001-certification)

- Information security policy and objectives (clauses 5.2 and 6.2)
- Information risk treatment process (clause 6.1.3)
- Risk treatment plan (clauses 6.1.3 e and 6.2)
- Risk assessment report (clause 8.2)
- Records of training, skills, experience and qualifications (clause 7.2)
- Monitoring and measurement results (clause 9.1)
- Internal audit programme (clause 9.2)
- Results of internal audits (clause 9.2)
- Definition of security roles and responsibilities (clauses A.7.1.2 and A.13.2.4);
- Inventory of assets (clause A.8.1.1);
- Acceptable use of assets (clause A.8.1.3);
- Access control policy (clause A.9.1.1);
- Operating procedures for IT management (clause A.12.1.1);
- Secure system engineering principles (clause A.14.2.5);
- Supplier security policy (clause A.15.1.1);
- Incident management procedure (clause A.16.1.5);
- Business continuity procedures (clause A.17.1.2);
- Statutory, regulatory and contractual requirements (clause A.18.1.1); and
- Logs of user activities, exceptions and security events (clauses A.12.4.1 and A.12.4.3).
- Results of the management review (clause 9.3)
- Results of corrective actions (clause 10.1)

# Appendix A - Threats

People take special care of their ePassports but a robust system accounts for exceptional incidents. These are explored below.

First, a note about the existing ePassport system to frustrate thieves and counterfeiters.

Interpol maintains a stolen and lost travel documents ([SLTD](https://www.interpol.int/en/How-we-work/Databases/Stolen-and-Lost-Travel-Documents-database)) database for law enforcement agencies. They also offer the [I-Checkit](https://www.interpol.int/en/How-we-work/Border-management/I-Checkit) service developed for "trusted partners in the private sector to conduct advanced passenger checks in real-time ... Research and development are ongoing in order to implement the I-Checkit solution within the financial sector."

I-Checkit requires nothing more than the ePassport number and issuing country, in compliance with Interpol's rules on the processing of data.

This is useful in the context here, but we should point out the differences in our threat vector. SLTD exists to prevent any one individual traversing borders illegally, and I-Checkit may help banks weed out mortgage fraud for example. It's all about identity. On the other hand NOT-A-BOT exists to enable proof-of-humanness and proof-of-uniqueness, and has no direct identity related function. When NOT-A-BOT is associated with wealth distribution, a single Sybil fraud may indeed advantage the fraudster. Nevertheless, in democratic and social media contexts, fraud would have to be conducted at scale (very many Sybils) to realise any significant benefit.

If the NOT-A-BOT service uses I-Checkit, the ePassport number will need to be communicated to the I-Checkit service.


## 1. Bad actor Bob uses Alice's ePassport — temporarily in his possession
Bob works at a hotel and uses other people's ePassports temporarily in his possession.

If Bob attempts this for an already registered ePassport, Alice's, then she is notified immediately as always. She realises someone at the hotel reception must be up to no good. She undoes this deviance (the NOT-A-BOT app with the earliest instance can always override) and makes a complaint with the hotel, both of which should deter Bob in the first place.

Note that Bob's attempt isn't immediately refused despite the hash of the ePassport being familiar, but rather put into a 30-day hold giving Alice (the previous use) time to respond.

Further, Bob is informed that he will have to repeat the ID registration process in 30 days, which he's unlikely to be able to do because Alice will have her ePassport back by then.

## 2. Bad actor Bob uses Alice's ePassport — stolen
If Bob has Alice's stolen ePassport, she can reply to the app notification to say it was not her, and also to communicate the theft once she has established she no longer has possession of her passport. She can also communicate loss or theft via the app without first being notified of someone else attempting to use the passport for NOT-A-BOT.

We invalidate any malicious attempts, and set an expiration for the claim secured for Alice via her now stolen ePassport in a further 90 days (in which time Alice will have secured a replacement and registered it with her app).

## 3. Bad actor Bob uses Alice's ePassport — stolen — and her phone has been stolen too
On logging into Ethereum World or any other service for which she has used NOT-A-BOT, Alice is notified of new uses. If she does not recognise these as her own uses, she 'delinks' these new verifications. If her phone is lost or stolen, she can notify us accordingly.

The corresponding hash of the ePassport is blacklisted. Alice gets a new ePassport. She logs into the NOT-A-BOT app and registers it.

## 4. Bad actor Bob uses an ePassport the hash for which is not yet in the system
Bob will succeed (subject to I-Checkit). Nevertheless, he will need a separate smartphone / boot per ePassport, so this is not trivial to scale to the democratic weight needed for most applications to have any value to Bob. And he will have to repeat the process for each stolen ePassport - smartphone / boot combination after 30 days and on-demand.

Moreover, it's only really worth Bob's time the bigger NOT-A-BOT gets, the more it is used in different contexts, by which time more and more ePassports that Bob might try to use will already be in the system

## 5. Alice tries to use her ePassport and finds it's already been used
Perhaps Bob at the hotel was first to use her ePassport and 30 days have not yet passed requiring a repeat verification? Or perhaps Bob still had access to the ePassport after 30 days? Or perhaps Charlie, Alice's husband, has taken a liberty?! Bob is bad, whereas Charlie has just been cheeky. We'll refer to both from here as Bob.

For previous threats (1., 2. and 3.) we've defaulted to allowing the earliest instance to override later instances. This helps Alice 'recover' as her instance predated Bob's. Here however we consider how to help Alice should Bob have used her ePassport first.

On using the NOT-A-BOT app for the first time, Alice is notified that her ePassport has already been used in our system. She is given the option to challenge this. She is told that such a challenge may involve a video conference with the support team at NOT-A-BOT, and is asked again if she would like to proceed. Alice confirms that she does.

Even though this matter may be (is very likely to be) resolved without recourse to a video conference, the fact that Alice is confident enough to want to proceed anyway indicates that she is likely legitimate.

If 30 days are not yet passed from the original registration, we can just let time do its thing. Otherwise ...

Bob will have received a notification that someone else is attempting to use the ePassport he's used. Now he receives a notification that his use is being challenged, and is asked whether he would like to contend this. He's told that contention may involve a video conference with the NOT-A-BOT support team. We can assume with some good degree of confidence that in the majority of instances Bob will balk at this point, especially if we say something like "no action will be taken if this is not your ePassport and you choose now to decline this challenge and relinquish the document record."

If Bob declines to contend, everything is sorted.

If Bob does not respond in 30 days, Alice's challenge is successful after that time.

The vast majority of such cases will resolve without the need for video conference. However, if Bob decides to contend, then both Alice and Bob are invited to select a time to join a video conference. The earliest slot is chosen, and our support team video conferences with one or the other. This involves a visual comparison (the ePassport is held up to the webcam) and the ePassport confirmed again with the NOT-A-BOT app based on the support team presenting a new QR code.

The video conference is recorded but immediately deleted upon a successful match.

## 6. Debbie has more than one ePassport
A small minority of people may have more than one ePassport — frequent travellers and those with more than one nationality.

If Debbie wanted to get more than one vote in a democratic DAO, she could try to use more than one ePassport to create two accounts considered unique. This is not necessarily straightforward if only because she will need two smartphones or a multi-boot Android.

But Debbie is persistent and she dual-boots her phone.

To fend off Debbie's fraud, the NOT-A-BOT app also communicates the hashes of additional information.

Mandatory data featured on all ePassports is indicated below by (m) and optional data that may be on some ePassports but not others by (o). [See Appendix B](https://docs.google.com/document/d/1A6J1N24LNvK0vmnvgilhlMt7IWqNeIS59mxc8RJb9JE/edit#heading=h.e60lwet6s6p6).

name (m) + date of birth (m)
name (m) + date of birth (m) + sex (m)
name (m) + date of birth (m) + place of birth (o)
name (m) + date of birth (m) + address (o)
name (m) + date of birth (m) + eye colour (o)
date of birth (m) + address (o)
date of birth (m) + place of birth (o) + address (o)
The multiple hashes of personal data are required because:

The combination of name and date of birth may not be unique
The name may have changed (e.g. maiden and married surname)
Address may have changed.
A hash match including only mandatory data may well be a false positive. Hash matches including optional data will be more reliable.

Absence of hash matches is not absence of fraud, nevertheless the odds of 'getting away with it' are greatly diminished. Communicate this process and associated penalty and the majority of this minority won't even 'try it on'.

The ePassport data cannot be computed from the hash. Determining identity from any hash is computationally intensive (and a client-side salt invalidates the purpose of collecting the hashes in the first place). Nevertheless, we could compute the hash given an individual’s personal information and easily determine if that hash exists in our database. For this reason, our system will make no record of any API calls corresponding to the table of hashes (otherwise, and with the cooperation of the platforms / networks / communities making the calls, one may be able to trace the accounts held by an individual).

## 7. Nora has no ePassport
NOT-A-BOT has no way to help Nora other than to help her find the instructions for obtaining an ePassport in her country. It may be feasible to integrate some Web-of-Trust mechanism once the primary mechanism here has scaled to hundreds of millions.


<!--- 

TODO: Appendix B Image: content/portfolio/technical/notabot/pictures/Appendix B.png

TODO: Add flows

--->
