extern crate xml;

use std::fs::File;
use std::io;
use std::io::{self, Write};

use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent};

fn main() {
    loop {
        println!("Give me an Ip and Mask.");

        let mut net_mask_input = String::new();

        io::stdin()
            .read_line(&mut net_mask_input)
            .expect("Failed to read Line");

        let split_input_to_get_mask: Vec<&str> = net_mask_input.split("/").collect();
        let split_input_to_get_ip: Vec<&str> = split_input_to_get_mask[0].split(".").collect();

        let mask: usize = match split_input_to_get_mask[1].trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let ip_last_octet: usize = match split_input_to_get_ip[split_input_to_get_ip.len() - 1]
            .trim()
            .parse()
        {
            Ok(num2) => num2,
            Err(_) => continue,
        };
        let binary_last_octet_from_ip = getting_last_octet_binary(ip_last_octet);
        let binary_last_octet_from_mask = getting_mask_octet(mask);

        getting_network_address_and_broadcast_address(
            binary_last_octet_from_ip,
            binary_last_octet_from_mask,
            split_input_to_get_ip,
        );

        break;
    }
}

fn getting_last_octet_binary(ip_last_octet: usize) -> Vec<usize> {
    let arr = vec![128, 64, 32, 16, 8, 4, 2, 1];
    let mut aux = 0;
    let binary_ip = 8;
    let mut count = 0;
    let mut binary_on_ip = vec![0; binary_ip];

    for walk in arr {
        aux += walk;

        if aux == ip_last_octet {
            binary_on_ip[count] = walk;
            break;
        }
        if aux > ip_last_octet {
            aux -= walk;
            binary_on_ip[count] = 0;
            count += 1;
        } else {
            binary_on_ip[count] = walk;
            count += 1;
            continue;
        }
    }

    return binary_on_ip;
}

fn getting_mask_octet(mask: usize) -> Vec<usize> {
    let mut num = 0;
    let byte = 8;
    let mut first_bit_from_the_last_octet = 24;
    let bit_in_octets = 32;
    let bits = vec![128, 64, 32, 16, 8, 4, 2, 1];
    let mut octet = vec![0; bit_in_octets];
    let mut last_octet = vec![0; byte];

    while num < bit_in_octets {
        for put_in_octet in &bits {
            if num >= mask {
                octet[num] = 0;
                num += 1;
            } else {
                octet[num] = *put_in_octet;
                num += 1;
            }
        }
    }

    for count in 0..7 {
        last_octet[count] = octet[first_bit_from_the_last_octet];
        first_bit_from_the_last_octet += 1;
    }

    return last_octet;
}

fn getting_network_address_and_broadcast_address(
    binary_last_octet_from_ip: Vec<usize>,
    binary_last_octet_from_mask: Vec<usize>,
    split_input_to_get_ip: Vec<&str>,
) {
    let octets = 4;
    let mut network_address = vec![0; octets];
    let mut broasdcast_address = vec![0; octets];
    let mut first_host_address = vec![0; octets];
    let mut last_host_address = vec![0; octets];
    let mut network_octet = 0;
    let broadcast_octet;
    let first_host_octet;
    let last_host_octet;
    let bits = vec![128, 64, 32, 16, 8, 4, 2, 1];

    for count in 0..7 {
        if binary_last_octet_from_ip[count] != 0 && binary_last_octet_from_mask[count] != 0 {
            network_octet += bits[count];
        }
    }

    broadcast_octet = network_octet + 31;
    first_host_octet = network_octet + 1;
    last_host_octet = broadcast_octet - 1;

    for count in 0..split_input_to_get_ip.len() {
        network_address[count] = split_input_to_get_ip[count].parse().unwrap();
        broasdcast_address[count] = split_input_to_get_ip[count].parse().unwrap();
        first_host_address[count] = split_input_to_get_ip[count].parse().unwrap();
        last_host_address[count] = split_input_to_get_ip[count].parse().unwrap();
    }

    network_address[octets - 1] = network_octet;
    first_host_address[octets - 1] = first_host_octet;
    last_host_address[octets - 1] = last_host_octet;
    broasdcast_address[octets - 1] = broadcast_octet;

    println!(
        "\n Network Address {:?}\n First Host Address{:?}\n Last Host Address{:?}\n Broadcast Address{:?}",
        network_address, first_host_address, last_host_address, broasdcast_address
    );

 
}
   //criar o arquivo xml e seguindo os padroes do arquivo do Douglas, segue modelo abaixo

// <DiscoveryJob identifier="DISCOVERY_JOB"><Description></Description><Schedule runnow="true"></Schedule>
// <DiscoveryOptionsList><DiscoveryOptions><MgmtProtocolList><MgmtProtocol>snmpv2c</MgmtProtocol>
// </MgmtProtocolList><useNmap>false</useNmap><doNotManageDevices>false
// </doNotManageDevices><useLoopBackIp>false</useLoopBackIp><Timeout>10
// </Timeout><IPRangeList><IPRange><Start>142.40.77.0</Start><End>142.40.77.255</End></IPRange>
// <IPRange><Start>142.40.78.0</Start><End>142.40.79.255</End></IPRange><IPRange><Start>142.40.80.0</Start>
// <End>142.40.80.255</End></IPRange><IPRange><Start>142.40.182.0</Start><End>142.40.183.255</End>
// </IPRange><IPRange><Start>142.40.184.0</Start><End>142.40.185.255</End></IPRange>
// <IPRange><Start>142.40.184.0</Start><End>142.40.187.255</End></IPRange></IPRangeList></DiscoveryOptions>
// </DiscoveryOptionsList></DiscoveryJob>
fn handle_event<W: Write>(w: &mut EventWriter<W>, line: String) -> Result<()> {
    let line = line.trim();
    let event: XmlEvent = if line.starts_with("+") && line.len() > 1 {
        XmlEvent::start_element(&line[1..]).into()
    } else if line.starts_with("-") {
        XmlEvent::end_element().into()
    } else {
        XmlEvent::characters(&line).into()
    };
    w.write(event);
}

fn create_file(){
    let mut file = File::create("output.xml").unwrap();

    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(&mut file);
    loop{
        print!("> "); output.flush().unwrap();
        let mut line = String::new();
        match input.read_line(&mut line){
            Ok(_) => {}
            Err(e)=> panic!("Write error: {}", e)
        },
        Err(e) => panic("Input error: {}", e)
    }
}