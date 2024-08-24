# Dynamic DNS Automation

[![Continuous Integration Workflow](https://github.com/tradecrit/dynamic-dns/actions/workflows/ci.yaml/badge.svg)](https://github.com/tradecrit/dynamic-dns/actions/workflows/ci.yaml)

## Description

Open source Dynamic DNS automation tool for updating DNS records on Cloudflare. This tool is designed to be run as a
scheduled job on a server or computer that has a public IP address. The tool will check the public IP address of the
server or computer and update the DNS record on Cloudflare if the public IP address has changed within a specific time
specified via the configuration environment.

The motivation behind this tool is to create an ultra-lightweight docker container that is fully open source, that can
work with any potential DNS provider. There was a surprising lack of documentation and automation around my specific use
case, so I decided to create this tool to help others in the same situation.

## Table of Contents
- [Dynamic DNS Automation](#dynamic-dns-automation)
  * [Description](#description)
  * [What is Dynamic DNS?](#what-is-dynamic-dns)
  * [Why use Dynamic DNS?](#why-use-dynamic-dns)
  * [Features](#features)
  * [Pre-requisites](#pre-requisites)
  * [Supported DNS Providers](#supported-dns-providers)
  * [Configuration](#configuration)
    + [Global Environment Variable Configuration](#global-environment-variable-configuration)
    + [Cloudflare](#cloudflare)
      - [Pre-requisites](#pre-requisites-1)
      - [Additional Environment Variables](#additional-environment-variables)
  * [Docker Tutorial](#docker-tutorial)
    + [Signup with Cloudflare](#signup-with-cloudflare)
      - [Pull Image](#pull-image)
      - [Create .env file for docker](#create-env-file-for-docker)
      - [Update .env file with your values](#update-env-file-with-your-values)
      - [Run the docker container](#run-the-docker-container)
  * [Docker Compose Tutorial](#docker-compose-tutorial)
    - [Run docker-compose](#run-docker-compose)
  * [Development Setup](#development-setup)
    + [Pre-requisites](#pre-requisites-2)
    + [Build](#build)
    + [Run](#run)
  * [License](#license)
  * [Contributing](#contributing)



## What is Dynamic DNS?

Dynamic DNS (DDNS) is a method of automatically updating a name server in the Domain Name System (DNS), often in real
time, with the active DDNS configuration of its configured hostnames, addresses, or other information. The term is used
to describe two different concepts. The first is "dynamic DNS updating" which refers to systems that are used to update
traditional DNS records without manual editing. These mechanisms are explained in RFC 2136, and use the TSIG mechanism
to provide security. The second kind of dynamic DNS permits lightweight and immediate updates often using an update
client, which do not use the RFC2136 standard for updating DNS records. These clients provide a persistent addressing
method for devices that change their location, configuration, or IP address frequently.

## Why use Dynamic DNS?

Dynamic DNS is useful for people who want to host a website, access a computer remotely, or create a home server.
Commonly ISP's will change your public IP address, which can be a problem if you are hosting a website or service that
you want to access remotely. Dynamic DNS allows you to create a domain name that points to your public IP address, and
automatically updates the DNS record if your public IP address changes.

## Features

- Update DNS record on Cloudflare
- Check public IP address of server or computer
- Update DNS record if public IP address has changed
- Small size, easy to use, and fully open source!

```
REPOSITORY    TAG       IMAGE ID       CREATED         SIZE
dynamic-dns   latest    6e34b4f35aa6   4 minutes ago   13.1MB
```

## Pre-requisites

- Docker

## Supported DNS Providers

- Cloudflare

More providers will be added in the future such as AWS Route53, and you can contribute by adding your own provider.

## Configuration

The dynamic-dns tool is configured via environment variables. The following environment variables are required to run
the tool independently of the DNS provider.

### Global Environment Variable Configuration

| Environment Variable       | Description                                                                       | Required | Default       | Example         |
|----------------------------|-----------------------------------------------------------------------------------|----------|---------------|-----------------|
| `REFRESH_INTERVAL_SECONDS` | The interval in seconds to check the public IP address and update the DNS record. | No       | `60`          | `300`           |
| `RUST_LOG`                 | The log level to use.                                                             | No       | `info`        | `debug`         |
| `ENVIRONMENT`              | Helper for observability to set environment                                       | No       | `development` | `dev`           |
| `DOMAINS`                  | The domain name to update the DNS record.                                         | Yes      |               | `example.com`   |
| `DNS_PROVIDER`             | The DNS provider to use.                                                          | Yes      |               | `cloudflare`    |
| `DNS_ENTRIES_TO_SYNC`      | Comma seperated list of subdomains to sync with the DNS Provider.                 | Yes      |               | `www,api,*.dev` |

### Cloudflare

Cloudflare is a popular DNS provider that offers a free tier for personal and light professional use. The Cloudflare
provider is enabled by default and can be configured via environment variables.

#### Pre-requisites

When you sign up with Cloudflare you need to get a few things to enable the cloudflare provider with the dynamic-dns
tool. Namely:

- Cloudflare account, this is your account numerical ID.
- Cloudflare API key, the API key used to interact with the Cloudflare REST API.
- Cloudflare Zone ID, the DNS zone ID that you want to update. This is typically the domain name you want to update,
  such as `example.com`.

#### Additional Environment Variables

| Environment Variable       | Description                                                | Required | Default | Example       |
|----------------------------|------------------------------------------------------------|----------|---------|---------------|
| `CLOUDFLARE_API_KEY`       | The API key used to interact with the Cloudflare REST API. | Yes      |         | `1234567890`  |
| `CLOUDFLARE_ZONE_ID`       | The DNS zone ID that you want to update.                   | Yes      |         | `example.com` |
| `CLOUDFLARE_PROXY_ENABLED` | Whether to enable the Cloudflare proxy for the DNS record. | Yes      | `true`  | `false`       |

## Docker Tutorial

This is a tutorial on how to use the dynamic-dns tool with Cloudflare. The tutorial will cover how to get the required
environment variables, and how to run the tool in a docker container.

### Signup with Cloudflare

1. Go to the [Cloudflare website](https://www.cloudflare.com/) and sign up for an account.
2. Add a domain to your Cloudflare account, when you add it you can see it in the Cloudflare dashboard.
3. Get your Cloudflare account ID, zone ID, and API key from the Cloudflare dashboard.

#### Pull Image

```bash
docker pull ghcr.io/tradecrit/dynamic-dns
```

#### Create .env file for docker

```bash
cp .env.example .env
```

#### Update .env file with your values

Update the .env file, so it looks similar the following:

```bash
DOMAIN="<your_domain>.com"

DNS_PROVIDER="cloudflare"
DNS_ENTRIES_TO_SYNC="*.example,example"

CLOUDFLARE_API_KEY="12345xyz12345"
CLOUDFLARE_ZONE_ID="123456789"
CLOUDFLARE_PROXY_ENABLED="true"
```

#### Run the docker container

```bash
docker run --env-file .env ghcr.io/tradecrit/dynamic-dns
```

That's it! The dynamic-dns tool will now check the public IP address of the server or computer every 60 seconds and
update the DNS record on Cloudflare if the public IP address has changed.

## Docker Compose Tutorial

This is a tutorial on how to use the dynamic-dns tool with Cloudflare using Docker Compose. The tutorial will cover how
to get the required environment variables, and how to run the tool in a docker container.

#### Run docker-compose

Run the following command to start the dynamic-dns tool with Docker Compose, make sure to pass the required environment
variables to the docker compose command.

```bash
CLOUDFLARE_API_KEY="<API_KEY>" CLOUDFLARE_ZONE_ID="<ZONE_ID>" docker-compose up -d
```

That's it! The dynamic-dns tool will now check the public IP address of the server or computer every 60 seconds and
update the DNS record on Cloudflare if the public IP address has changed.

## License

[GPLv3](https://www.gnu.org/licenses/gpl-3.0.html)

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.
