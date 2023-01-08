# Call whois via TOR
arti_whois is a whois service caller, works with tor sockets. developed against [ban](https://www.ripe.net/manage-ips-and-asns/db/support/documentation/ripe-database-acceptable-use-policy/why-did-i-receive-an-error-201-access-denied) 

arti is #tor library for [#rust](https://gitlab.torproject.org/tpo/core/arti) 

#how to use
call it with curl or [frizz](https://crates.io/crates/frizz) example call below
ip whois call
```bash
curl "127.0.0.1:8016/whois?ip=1.1.1.1&source=http://localhost:8080/whois.htm" -v
*   Trying 127.0.0.1:8016...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 8016 (#0)
> GET /whois?ip=1.1.1.1&source=http://localhost:8080/whois.htm HTTP/1.1
> Host: 127.0.0.1:8016
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: text/plain; charset=utf-8
< content-length: 2683
< date: Sun, 08 Jan 2023 14:31:26 GMT
< 

#
# ARIN WHOIS data and services are subject to the Terms of Use
# available at: https://www.arin.net/resources/registry/whois/tou/
#
# If you see inaccuracies in the results, please report at
# https://www.arin.net/resources/registry/whois/inaccuracy_reporting/
#
# Copyright 1997-2023, American Registry for Internet Numbers, Ltd.
#


#
# Query terms are ambiguous.  The query is assumed to be:
#     "n 1.1.1.1"
#
# Use "?" to get help.
#

NetRange:       1.0.0.0 - 1.255.255.255
CIDR:           1.0.0.0/8
NetName:        APNIC-1
NetHandle:      NET-1-0-0-0-1
Parent:          ()
NetType:        Allocated to APNIC
OriginAS:       
Organization:   Asia Pacific Network Information Centre (APNIC)
RegDate:        
Updated:        2010-07-30
Comment:        This IP address range is not registered in the ARIN database.
Comment:        For details, refer to the APNIC Whois Database via
Comment:        WHOIS.APNIC.NET or http://wq.apnic.net/apnic-bin/whois.pl
Comment:        ** IMPORTANT NOTE: APNIC is the Regional Internet Registry
Comment:        for the Asia Pacific region. APNIC does not operate networks
Comment:        using this IP address range and is not able to investigate
Comment:        spam or abuse reports relating to these addresses. For more
Comment:        help, refer to http://www.apnic.net/apnic-info/whois_search2/abuse-and-spamming
Ref:            https://rdap.arin.net/registry/ip/1.0.0.0

ResourceLink:  http://wq.apnic.net/whois-search/static/search.html
ResourceLink:  whois.apnic.net


OrgName:        Asia Pacific Network Information Centre
OrgId:          APNIC
Address:        PO Box 3646
City:           South Brisbane
StateProv:      QLD
PostalCode:     4101
Country:        AU
RegDate:        
Updated:        2012-01-24
Ref:            https://rdap.arin.net/registry/entity/APNIC

ReferralServer:  whois://whois.apnic.net
ResourceLink:  http://wq.apnic.net/whois-search/static/search.html

OrgTechHandle: AWC12-ARIN
OrgTechName:   APNIC Whois Contact
OrgTechPhone:  +61 7 3858 3188 
OrgTechEmail:  search-apnic-not-arin@apnic.net
OrgTechRef:    https://rdap.arin.net/registry/entity/AWC12-ARIN

OrgAbuseHandle: AWC12-ARIN
OrgAbuseName:   APNIC Whois Contact
OrgAbusePhone:  +61 7 3858 3188 
OrgAbuseEmail:  search-apnic-not-arin@apnic.net
OrgAbuseRef:    https://rdap.arin.net/registry/entity/AWC12-ARIN


#
# ARIN WHOIS data and services are subject to the Terms of Use
# available at: https://www.arin.net/resources/registry/whois/tou/
#
# If you see inaccuracies in the results, please report at
# https://www.arin.net/resources/registry/whois/inaccuracy_reporting/
#
# Copyright 1997-2023, American Registry for Internet Numbers, Ltd.
#
* Connection #0 to host 127.0.0.1 left intact
```
empty ip call, returning 200 for health checks.
```bash
curl 127.0.0.1:8016/whois -v
*   Trying 127.0.0.1:8016...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 8016 (#0)
> GET /whois HTTP/1.1
> Host: 127.0.0.1:8016
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< content-type: text/plain; charset=utf-8
< content-length: 8
< date: Sun, 08 Jan 2023 14:34:28 GMT
< 
* Connection #0 to host 127.0.0.1 left intact
empty ip
```

wrong ip call
```bash
curl "127.0.0.1:8016/whois?ip=1.1&source=http://localhost:8080/whois.htm" -v
*   Trying 127.0.0.1:8016...
* TCP_NODELAY set
* Connected to 127.0.0.1 (127.0.0.1) port 8016 (#0)
> GET /whois?ip=1.1&source=http://localhost:8080/whois.htm HTTP/1.1
> Host: 127.0.0.1:8016
> User-Agent: curl/7.68.0
> Accept: */*
> 
* Mark bundle as not supporting multiuse
< HTTP/1.1 400 Bad Request
< content-type: text/plain; charset=utf-8
< content-length: 8
< date: Sun, 08 Jan 2023 15:20:27 GMT
< 
* Connection #0 to host 127.0.0.1 left intact
wrong ip
```

# future implementation
right now there is no tor connection refresher, it would be nice to refresh tor connection if it gets "access denied" [check](https://github.com/ozkanpakdil/rust-examples/issues/4)