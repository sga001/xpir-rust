#include <iostream>
#include "pir.hpp"
#include <iomanip>
#include <time.h>
#define BILLION 1000000000L
#define MILLION (1.0*1000000L)
#define KILO (1.0*1024L)

int main(int argc, char *argv[])
{
  if (argc < 4) {
    cout<<"Pass params: [db_size] [chosen_idx] [alpha] [d]"<<endl;
    exit(1);
  }

  uint64_t num_db_entries = uint64_t(atoi(argv[1])); //1024*1024; //8192;
  uint64_t chosen_idx = uint64_t(atoi(argv[2]));
  uint64_t alpha = uint64_t(atoi(argv[3]));
  uint64_t d = uint64_t(atoi(argv[4]));
  
  
  uint64_t diff;
  struct timespec start, end;

  
  uint64_t num_bytes_per_entry = 288;
  uint64_t len_total_bytes = num_db_entries*num_bytes_per_entry;

  char *stream = new char[len_total_bytes];
  
  for (int i=0; i<num_db_entries; i++) {
    for (int j=0; j<num_bytes_per_entry; j++) {
       stream[i*num_bytes_per_entry+j] = (char)i;
    }
  }


  PIRClient *client = (PIRClient*) cpp_client_setup(len_total_bytes, num_db_entries, alpha, d);
  XPIRServer *server = (XPIRServer*) cpp_server_setup(len_total_bytes, stream, num_db_entries, alpha, d);

  uint64_t len, len_element, rlen_element, rlen;
  uint64_t num_trials = 1;
  cout<<"Chosen idx: "<<chosen_idx<<endl;
  for (int i=0; i< num_trials; i++) {

    clock_gettime(CLOCK_MONOTONIC, &start);
    char *query = cpp_client_generate_query(client, chosen_idx, &len, &len_element);
    clock_gettime(CLOCK_MONOTONIC, &end);
    diff = BILLION * (end.tv_sec - start.tv_sec) + end.tv_nsec - start.tv_nsec;
    cout<<"time_query "<<diff/MILLION<<" ms"<<endl;
    cout<<"len_query "<<len/KILO<<" kb"<<endl;

    if (len == 0) {
      cout << "FUCK"<<endl;
    }

    clock_gettime(CLOCK_MONOTONIC, &start);
    char *response = cpp_server_process_query(server, query, len, len_element, &rlen, &rlen_element); 
    clock_gettime(CLOCK_MONOTONIC, &end);
    diff = BILLION * (end.tv_sec - start.tv_sec) + end.tv_nsec - start.tv_nsec;
    cout<<"time_answer "<<diff/MILLION<<" ms"<<endl;
    cout<<"len_answer "<<rlen/KILO<<" kb"<<endl;

    clock_gettime(CLOCK_MONOTONIC, &start);
    char *decoded_response = cpp_client_process_reply(client, response, rlen, rlen_element, &len); 
    clock_gettime(CLOCK_MONOTONIC, &end);
    diff = BILLION * (end.tv_sec - start.tv_sec) + end.tv_nsec - start.tv_nsec;
    cout<<"time_decode "<<diff/MILLION<<" ms"<<endl;
    cout<<"len_decode "<<len<<" bytes"<<endl;

    for (int i=0; i<len; i++) {
      cout<<int(decoded_response[i]);
    }
    cout<<endl;
    
    cpp_buffer_free(query);
    cpp_buffer_free(response);
    cpp_buffer_free(decoded_response);
 
  }

  return 0;
}
