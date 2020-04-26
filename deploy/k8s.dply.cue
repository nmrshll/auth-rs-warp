package k8s

deployment <Name>: {
	apiVersion: *"extensions/v1beta1" | string
	kind:       "Deployment"
	metadata name: *Name | string
	spec: {
		replicas: *1 | int
		selector matchLabels app: *Name | string
		template: {
			metadata labels app: *Name | string
			spec containers: [{name: *Name | string }]
		}
	}
}
service <Name>: {
	apiVersion: *"v1" | string
	kind:       "Service"
	metadata name: *Name | string
	metadata annotations "dev.okteto.com/auto-ingress": *"true" | bool
	spec: {
		type: *"ClusterIP" | string
		selector app: *Name | string
		ports: [...{
			name: *Name | string
			port: int
		}]
	}
}
configMap <Name>: {
	apiVersion: *"v1" | string
	kind: "ConfigMap"
	metadata name: *"config-\(Name)" | string
	metadata labels app: *Name | string
}
persistentVolumeClaim <Name>: {
	apiVersion: *"v1" | string
	kind: "PersistentVolumeClaim"
	metadata name: *"pv-claim-\(Name)" | string
	spec accessModes: ["ReadWriteOnce"]
	spec resources requests storage: *"100Mi" | string
}
job <Name>: {
	apiVersion: *"batch/v1" | string
	kind: "Job"
	metadata name: *Name | string
	spec template spec restartPolicy: *"Never" | string
	spec template spec containers: [{ name: *Name | string }]
	spec backoffLimit: *4 | int
}
ingress <Name>: {
	apiVersion: "networking.k8s.io/v1beta1"
	kind: "Ingress"
	metadata name: "ingress-\(Name)"
	spec rules: [{  }]
}


//////////////////


deployment api: {
	spec template spec containers: [{
		image: "docker.pkg.github.com/pacio-core/tender-apps/smartlike-node:latest"
		envFrom: [{ configMapRef name: "config-postgres" }]
	}]
	spec template spec imagePullSecrets:[{
		// manually: add credentials to cluster: https://kubernetes.io/docs/tasks/configure-pod-container/pull-image-private-registry/
		name: "regcred-tender-apps"
	}]
}
service api: {
	spec type: "LoadBalancer"
		metadata annotations "dev.okteto.com/auto-ingress": false
	spec ports: [{ port: 7655 },{ name: "ws", port: 7654 }]
}

deployment postgres: {
	spec template spec containers: [{
		image: "postgres:10.4"
		ports: [{ containerPort: 5432 }]
		envFrom: [{ configMapRef name: "config-postgres" }]
		volumeMounts: [{ 
			name:  "volume-postgres-data"
			mountPath: "/var/lib/postgresql"
			subPath: "data"
		}]
	}]
	spec template spec volumes: [{
		name: "volume-postgres-data"
		persistentVolumeClaim claimName: "pv-claim-postgres"
	}]
}
service postgres: {
	spec ports: [{ port: 5432 }]
}
configMap postgres data: {
	POSTGRES_DB: "postgresdb"
	POSTGRES_USER: "postgresuser"
	POSTGRES_PASSWORD: "postgrespassword"
	POSTGRES_HOST: "postgres:5432"
}
persistentVolumeClaim postgres: {}

deployment adminer: {
	spec template spec containers: [{
		image: "adminer:4.2.5"
		ports: [{ containerPort: 8080 }]
	}]
}
service adminer: {
	spec ports: [{
		port: 7897
		targetPort: 8080
	}]
}

job migrations: {
	spec template spec containers: [{
		image: "docker.pkg.github.com/pacio-core/tender-apps/smartlike-node-migrations:latest"
		envFrom: [{ secretRef name: "smartlike-node-postgres-db" }]
	}]
	spec template spec imagePullSecrets:[{ name: "regcred-tender-apps" }]
}





