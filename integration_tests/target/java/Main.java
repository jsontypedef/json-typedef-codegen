package com.example;

import java.util.Scanner;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;

public class Main {
    public static void main(String[] args) throws JsonProcessingException {
        ObjectMapper objectMapper = new ObjectMapper();
        objectMapper.registerModule(new JavaTimeModule());
        objectMapper.disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS);

        Scanner scanner = new Scanner(System.in);

        while (scanner.hasNextLine()) {
            MAIN_CLASS value = objectMapper.readValue(scanner.nextLine(), MAIN_CLASS.class);
            System.out.println(objectMapper.writeValueAsString(value));
        }
    }
}
