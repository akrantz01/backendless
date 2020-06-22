# [Backendless](https://devpost.com/software/backendless)
###### By Alex Krantz

## Introduction
It is an indisputable fact that every web, mobile, or desktop application requires a backend, whether it is a custom one or a 3rd party's API. The backend provides data persistence, data processing, file storage, authentication, and more. With all these requirements for a backend, it is no wonder backends are so complicated to build, deploy, and maintain. As development speed is getting progressively more important, your team needs to be able to move as fast as possible to get your product to market.

Modern frameworks like Django, Flask, ExpressJS, and Spring merely abstract the network and routing layers, still requiring you to handle connecting to your database and storage services of choice. These frameworks abstract nothing from you, giving you great flexibility but creating unnecessary complexity within your backend.

Providing more abstractions are services like Google Firebase, which provides authentication, storage, hosting, and database access in one unified system. However, it still requires you to handle routing and integrating with their complex APIs. These services force you to learn the intricacies of all their bundled systems to create your app. Wouldn't it be nice to have a simple, straight forward interface to all your platform's services?

When building your application, you should be able to focus more on the user experience and user interface, instead of messing around with the complexities of your framework or backend-as-a-service of choice. Introducing **Backendless**: the most straightforward and accessible backend-as-a-service platform.


## What it does
Backendless provides the simplest, batteries included backend for your next project without writing a single line of code. All your backend's routes get configured through a YAML file with a consistent and straightforward syntax. Rather than having a bunch of different terms that reference different parts of your application, we only have two that you need to worry about: routes and handlers. Your routes specify where what handlers a given request should go to, and your handlers determine how the request should be acted on.

Backendless provides access to the necessary services your application needs, such as a database and file storage. We support all the request parameters that you're familiar with such as query parameters, path parameters, headers, and JSON request bodies. As with any web application, it is a necessity to validate everything coming into your service which is why we support requiring specific types on path parameters and JSON Schema validation for request bodies. We also only provide your request context the parameters you specify, allowing you to focus on the data that matters most.

## How I built it
Backendless was built using Rust for the primary API and Python for the user-supplied API. For Rust, I used the Actix-Web framework to provide general user actions such as user authentication, project management, and deployment management. For Python, I used the Starlette framework since it works at a lower level than Flask or Django and allows easy runtime modification of the routing tables.

Backendless is hosted on Google Cloud Platform with the primary and user-supplied APIs running in containers on Cloud Run to allow for auto-scaling based on request load. I'm using a Cloud SQL for PostgreSQL database as the primary database for user, project, and deployment information since the data is highly relational and has a fixed schema. I'm using a Cloud Firestore instance for the user API generated data with namespaces for each project since I am unable to know in advance what types of data or the schema of how the data is going to be stored. It also provides the most flexibility for the user. I am also using a Memorystore for Redis instance for communication between the primary Rust API and the user API runtime. Finally, the user uploaded files and the website are stored in Cloud Storage buckets, which provide easy access to the data, as well as static file hosting for the site.

## Challenges I ran into
1. When a user creates a deployment, they are able to upload static files for the service. These files are compressed into a zip file to reduce network bandwidth. Unzipping and uploading the files to the Cloud Storage Bucket all in memory, proved to be quite a challenge.
2. Getting the services to all be able to communicate proved to be a challenge since it was not very clear in the GCP documentation. In the end, it turned out there was a dedicated service for connecting serverless applications like Cloud Run to a VPC network.
3. Modifying the Starlette routing table during runtime proved to be a challenge since it required digging through the library internals to figure out how it worked.

## What I learned
I learned how to deploy a complex application that depends on multiple services to Google Cloud Platform. This also increased my understanding of how Docker works since the Cloud Run services had to be in Docker images.

## What's next for Backendless
Looking to the future, there are many things that can be improved with both the user experience and underlying service.

Starting with the user experience, all interaction with the service is done via the command line which is not particularly user friendly. I would like to add a frontend that allows users to see their projects and deployments. The frontend could also provide a graphical interface for defining APIs with a drag-and-drop interface. This would considerably lower the bar to entry allowing even the most inexperienced coders with an idea to bring it to fruition.

As for the underlying service, the user-provided functions themselves could use some major improvement by either changing the language they're written in or using code generation to achieve compiled and static services isolated from everything else. Furthermore, the "standard library" could definitely be expanded to allow for loop constructs, authentication, and integration with 3rd-party APIs like SendGrid or Twilio. It would also be advantageous to include support for other request body types such as multipart forms or GraphQL. I also plan to switch from a Redis instance to a Cloud PubSub instance because Redis is designed for more than just pub/sub and it would also be considerably cheaper.