# OSI model

+ physical layer
  + Defines data transmission in electrical and physical terms
  + Can be by wire, fiber optic, or wireless medium
+ Data link Layer
  + Defines transmission of data between two nodes connected by physical medium
  + Deals with prioritization between multiple wires trying simultaneous access
+ Network Layer
  + Defines how packets are transmitted between networks
  + + Need to define how to identify hosts and networks
+ Transport Layer
  + Defines mechanisms to deliver variable length messages to hosts in same or different networks
  + Defines a stream of packets that the receiver listens to
+ Session Layer
  + Defines communication between applications running on hosts
  + Need to differentiate between applications running on same host
  + Delivering packets to them
+ presentation Layer
  + Defines common formats for data representation
  + Takes care of security
+ Application Layer
  + Defines how user-centric applications should send adn receive data
  + Web browser using HTTP to talk to a web server

## addressing Modes

+ Ethernet Address
  + Also known as MAC address
  + Assigned 48 bit long unique identifier
  + Programmed by network card manufacture
  + Standard way - six groups of two hexadecimal digits
  + 01-23-45-67-89-ab-cd-ef or 01:23:45:67:89:ab:cd:ef
+ IP Address
  + Assigned to each device in an IP network
  + IPv4 - 32 bits and IPv6 - 128 bites
  + use a CIDR notation - 192.168.100.1/26(IPv4)
  + 2**23-26 = 2**6 = 64(192.168.100.0 to 192.168.100.63)
  + IAMA assigns blocks of IP addresses to organizers
  + IP addresses reserved for various purposes
  + assigned by DHCP in a home network
+ Autonomous System Number
  + 32-bit number
  + Used to identify autonomous systems
  + Assigned and maintained by IANA

## How NDS works?

+ system call - get addr info
+ asks the OS to resolve the name
+ Local DNS server is configured in /etc/resolv.conf
+ Server knows the address:
  + Prepares a response
  + Marks as authoritative
  + Send its back to local DNS server
+ Server does not know address:
  + Name resolution fails
  + Sends back special response(NXDOMAIN)

### Advantages of DNS

+ Small packets - small question, answer, and control information
+ Ideal candidate for using UDP
+ There is an option to fail back to TCP if transport is unreliable

## Common service models

## Connection-Oriented Service

+ When a virtual connection is negotiated before sending the actual data
+ During setup process, connection parameters must be agreed upon
+ Analogous to old wired telephone systems
+ Modern example is TCP
+ Consists of header and data section

> TCP Header Format

#### Flags

+ SYN - Triggers a synchronization of sequence numbers
+ ACK - indicates that receiver should care about acknowledgement number
+ FIN - Starts process of tearing down a connection
+ RST - Resets a connection in case of error

### Three-Way Handshake


#### When Does Connection-Oriented Service Model Work?

+ Large number of use cases
+ Case where overhead is significant or unnecessary - video streaming
  + Few missing packets do not cause a problem
  + Hence, they prefer a connectionless model

## Connectionless Service

+ Messages bear no relation to one another
+ Don't need connection negotiation step
+ Example - UDP
+ No guarantee of sequence or reliability of transmitted messages
+ Protocol above UDP is free to implement reliability

# install 

> rustup install nightly beta
> 
> cargo new --bin hello-rust
> 
> cargo search term


> brew install helix
