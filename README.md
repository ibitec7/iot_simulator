This project focuses on simulating IoT sensor data streams to Apache Kafka as producers.

## How to Deploy
1. Start a minikube cluster:
```
    minikube start --vm-driver=docker --cpus=<cpus> --memory=<mem>
```

2. Deploy the ConfigMaps
```
    kubectl apply -f deployment/configmap/
```

3. Deploy the Kafka cluster and monitor:
```
    kubectl apply -f deployment/kafka/
    kubectl get pods -w
```

4. Once running, deploy the consumers and monitor:
```
    kubectl apply -f deployment/consumer/
    kubectl get pods -w
```

5. Deploy the producers once running and monitor:
```
    kubectl apply -f deployment/producer/
    kubectl get pods -w
```

6. Listen in on any consumer by printing it's logs:
- First get the consumer pod ids:
```
    kubectl get pods | grep consumer
```
- Then use listen the log of any pod by copying from the NAME field:
```
    kubectl logs <NAME> -f
```

