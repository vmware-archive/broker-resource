# broker-resource

*a resource for cloud foundry service brokers*

## about

This resource allows a task to use a service instance or service binding as
defined by the [Cloud Foundry V2 Service Broker API][service-broker-api].

This is a rare case of an input/output only resource as there is no information
given about the version of a service from the broker. The input of this resource
checks out a single instance and binding from a resource and the output deletes
and cleans up the service and binding.

[service-broker-api]: http://docs.cloudfoundry.org/services/api.html
