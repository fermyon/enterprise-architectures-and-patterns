package main

import (
	"encoding/json"
	"fmt"
	"os"
	"os/signal"
	"strings"
	"syscall"
	"time"

	"database/sql"

	mqtt "github.com/eclipse/paho.mqtt.golang"
	_ "github.com/mattn/go-sqlite3"
)

const (
	dbKind             = "sqlite3"
	magicalWord        = "foobar"
	sqlReportJobStatus = "UPDATE Jobs SET Status=?, Result=? WHERE Id=?"
)

const (
	Pending   JobStatus = 0
	Running   JobStatus = 1
	Succeeded JobStatus = 2
	Failed    JobStatus = 3
)

var cfg Config

func init() {
	cfg = LoadConfig()
}

func main() {
	opts := mqtt.NewClientOptions()
	opts.AddBroker(cfg.MqttAddress)
	opts.SetClientID(cfg.MqttClientId)
	opts.SetUsername(cfg.MqttUsername)
	opts.SetPassword(cfg.MqttPassword)

	opts.OnConnect = connectHandler
	opts.OnConnectionLost = connectLostHandler
	client := mqtt.NewClient(opts)
	if token := client.Connect(); token.Wait() && token.Error() != nil {
		panic(token.Error())
	}
	if token := client.Subscribe(cfg.Topic, 2, messagePubHandler); token.Wait() && token.Error() != nil {
		panic(fmt.Sprintf("Error subscribing to topic: %s", token.Error()))
	}

	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan

	client.Unsubscribe(cfg.Topic)
	client.Disconnect(250)
}

var connectHandler mqtt.OnConnectHandler = func(client mqtt.Client) {
	fmt.Println("Connected")
}

var connectLostHandler mqtt.ConnectionLostHandler = func(client mqtt.Client, err error) {
	fmt.Printf("Connect lost: %v", err)
}

var messagePubHandler mqtt.MessageHandler = func(client mqtt.Client, msg mqtt.Message) {
	var job Job
	json.Unmarshal(msg.Payload(), &job)
	err := report_job_status(job.Id, "", Running)
	if err != nil {
		fmt.Printf("Error while reporting Job Status: %s\n", err)
		return
	}
	if strings.ToLower(job.Input) == magicalWord {
		fmt.Printf("Received '%s' as input for job, will fail\n", magicalWord)
		err = report_job_status(job.Id, fmt.Sprintf("Received '%s' as input", magicalWord), Failed)
		if err != nil {
			fmt.Printf("Error while reporting Job Status: %s\n", err)
			return
		}
		fmt.Println("Failed. Done.")
		return
	}
	fmt.Printf("Job received starting work... (will take 2mins)\n")
	time.Sleep(2 * time.Minute)
	_ = report_job_status(job.Id, fmt.Sprintf("%s!", job.Input), Succeeded)
	fmt.Println("Succeeded. Done.")
}

func report_job_status(id string, result string, status JobStatus) error {
	db, err := sql.Open(dbKind, cfg.DbFilePath)
	if err != nil {
		return err
	}
	defer db.Close()
	_, err = db.Exec(sqlReportJobStatus, status, result, id)
	return err
}

type Job struct {
	Id     string    `json:"id"`
	Input  string    `json:"input"`
	Result string    `json:"result"`
	Status JobStatus `json:"status"`
}

type JobStatus int32

type Config struct {
	DbFilePath   string
	MqttAddress  string
	MqttClientId string
	MqttUsername string
	MqttPassword string
	Topic        string
}

func LoadConfig() Config {
	return Config{
		DbFilePath:   "../data/jobs.db",
		MqttAddress:  "mqtt://localhost:1883",
		MqttClientId: "non-spin-worker",
		MqttUsername: "",
		MqttPassword: "",
		Topic:        "jobs/new",
	}
}
